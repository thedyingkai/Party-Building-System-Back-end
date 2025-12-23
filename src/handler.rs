use axum::{Json, Router, response::IntoResponse};
use sqlx::PgPool;

use crate::error::AppError;
use crate::features::user::handler as user_routes;
use crate::model::ResultWrapper;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

// 路由装配（供 main 使用）
pub fn routes() -> Router<AppState> {
    user_routes::router()
}

// 统一错误转 JSON
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = self.code_msg();
        let body = Json(ResultWrapper::<()>::err(code, msg));
        body.into_response()
    }
}
