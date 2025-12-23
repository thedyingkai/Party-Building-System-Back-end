use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{Request, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::future::Future;

use crate::common::error::ApiError;
use crate::model::UserDto;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
}

#[derive(Clone, Debug)]
pub struct AuthUser(pub UserDto);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let user = parts.extensions.get::<UserDto>().cloned();
        async move {
            user.map(AuthUser)
                .ok_or_else(|| ApiError::Unauthorized.into_response())
        }
    }
}

pub async fn jwt_auth(mut req: Request<Body>, next: Next) -> Result<Response, Response> {
    let path = req.uri().path();
    let public_paths = [
        "/login",
        "/register",
        "/file/download",
        "/column/selectAllToShow",
        "/article/selectArticleBycoid",
        "/article/selectArticleByid",
        "/article/selectArticleByuid",
        "/article/selectArticleBybranch",
        "/user/setgid",
    ];
    if public_paths.iter().any(|p| path.starts_with(p)) {
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get("token")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            req.uri().query().and_then(|q| {
                q.split('&').find_map(|kv| {
                    let mut it = kv.splitn(2, '=');
                    match (it.next(), it.next()) {
                        (Some("token"), Some(val)) => Some(val.to_string()),
                        _ => None,
                    }
                })
            })
        });

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return Err(ApiError::Unauthorized.into_response()),
    };

    let pool = req.extensions().get::<PgPool>().cloned().ok_or_else(|| {
        ApiError::Internal("Missing PgPool in request extensions".into()).into_response()
    })?;

    let claims = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(b"dummy"),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(TokenData { claims, .. }) => claims,
        Err(_) => return Err(ApiError::Unauthorized.into_response()),
    };

    let user_id: i64 = claims.aud.parse().unwrap_or(0);
    let user = sqlx::query_as::<_, UserDto>("SELECT * FROM djpt.user_view WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ApiError::Internal(format!("DB error: {e}")).into_response())?;

    let user = match user {
        Some(u) => u,
        None => return Err(ApiError::Unauthorized.into_response()),
    };

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(user.password.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| ApiError::Unauthorized.into_response())?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
