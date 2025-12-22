use axum::{extract::State, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::model::{ResultWrapper, UserDto};
use crate::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn index_handler() -> impl IntoResponse {
    Json(ResultWrapper::<()>::ok_msg("这里是主页"))
}

pub async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<Json<ResultWrapper<Vec<UserDto>>>, AppError> {
    let svc = UserService::new(&state.pool);
    let data = svc.list_users().await?;
    Ok(Json(ResultWrapper::ok(data)))
}

// 路由装配（供 main 使用）
pub fn routes() -> axum::Router<AppState> {
    use axum::routing::get;

    axum::Router::new()
        .route("/index", get(index_handler))
        .route("/users", get(list_users_handler))
}

// 统一错误转 JSON
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = self.code_msg();
        let body = Json(ResultWrapper::<()>::err(code, msg));
        body.into_response()
    }
}