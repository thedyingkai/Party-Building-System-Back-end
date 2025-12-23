use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody {
    code: i32,
    msg: String,
    data: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, msg) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, 400, m),
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, 404, m),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, 401, "Unauthorized".to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, 403, "Forbidden".to_string()),
            ApiError::Conflict(m) => (StatusCode::CONFLICT, 409, m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, 500, m),
        };

        let body = axum::Json(ErrorBody {
            code,
            msg,
            data: None,
        });
        (status, body).into_response()
    }
}
