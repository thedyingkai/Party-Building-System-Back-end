use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn ok_none() -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: None,
        }
    }

    pub fn fail(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResult<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Serialize, Default)]
pub struct PageResult<T> {
    pub total: i64,
    pub pageNum: i64,
    pub pageSize: i64,
    pub list: Vec<T>,
}

impl<T: Serialize> IntoResponse for PageResult<T> {
    fn into_response(self) -> Response {
        axum::Json(super::result::ApiResult::ok(self)).into_response()
    }
}
