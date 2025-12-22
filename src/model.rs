use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ResultWrapper<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ResultWrapper<T> {
    pub fn ok(data: T) -> Self {
        Self { code: "200".to_string(), msg: "请求成功".to_string(), data: Some(data) }
    }
    pub fn ok_msg(msg: &str) -> Self {
        Self { code: "200".to_string(), msg: msg.to_string(), data: None }
    }
    pub fn err(code: String, msg: String) -> Self {
        Self { code, msg, data: None }
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub total: i32,
    pub list: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub user_id: i64,
    pub user_name: String,
    pub password: String,
    pub points: i64,
    pub role_id: i64,
    pub role_name: String,
    pub role_permissions: String,
}