use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize)]
pub struct ApiResp<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResp<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: "200".to_string(),
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_msg(msg: impl Into<String>) -> Self {
        Self {
            code: "200".to_string(),
            msg: msg.into(),
            data: None,
        }
    }

    pub fn fail(msg: impl Into<String>) -> Self {
        Self {
            code: "500".to_string(),
            msg: msg.into(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PageResp<T> {
    pub total: i64,
    #[serde(rename = "pageNum")]
    pub page_num: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub list: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    #[serde(rename = "pageNum")]
    pub page_num: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub uname: Option<String>,
    pub cid: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub id: i32,
    pub cid: Option<i32>,
    pub gid: Option<i32>,
    pub bid: Option<i32>,
    pub deid: Option<i32>,
    pub uname: String,
    pub psw: String,
    pub points: Option<i32>,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub permissions: Option<String>,
    pub cname: Option<String>,
    pub branch_name: Option<String>,
    pub group_name: Option<String>,
    pub depart_name: Option<String>,
    pub sector_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserUpsert {
    pub id: Option<i32>,
    pub cid: Option<i32>,
    pub gid: Option<i32>,
    pub bid: Option<i32>,
    pub deid: Option<i32>,
    pub uname: String,
    pub psw: String,
    pub points: Option<i32>,
    pub username: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "snake_case")]
pub struct User2 {
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
    pub points: i32,
    pub role_id: i32,
    pub role_name: Option<String>,
    pub role_permissions: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User2Login {
    pub user_name: String,
    pub password: String,
}
