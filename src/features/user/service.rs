use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sqlx::PgPool;

use crate::error::AppResult;

use super::models::{ApiResp, PageQuery, PageResp, User2, User2Login, UserUpsert, UserView};
use super::repo::{User2Repo, UserViewRepo};

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

#[derive(serde::Serialize)]
struct JwtClaims {
    aud: String,
    exp: usize,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn user_repo(&self) -> UserViewRepo {
        UserViewRepo::new(&self.pool)
    }

    fn user2_repo(&self) -> User2Repo {
        User2Repo::new(&self.pool)
    }

    pub async fn hello(&self) -> AppResult<ApiResp<String>> {
        Ok(ApiResp::success("success".to_string()))
    }

    pub async fn index(&self) -> AppResult<ApiResp<String>> {
        Ok(ApiResp::success("这里是主页".to_string()))
    }

    pub async fn login_user(&self, uname: &str, psw: &str) -> AppResult<ApiResp<UserView>> {
        let user = self
            .user_repo()
            .get_by_uname(uname)
            .await?
            .ok_or_else(|| anyhow!("用户{}不存在", uname))?;
        if user.psw != psw {
            return Ok(ApiResp::fail(format!("密码 {} 不正确", psw)));
        }
        Ok(ApiResp::success(user))
    }

    pub async fn register_user(&self, u: &UserUpsert) -> AppResult<ApiResp<UserView>> {
        self.user_repo().insert(u).await?;
        let created = self
            .user_repo()
            .get_by_uname(&u.uname)
            .await?
            .ok_or_else(|| anyhow!("创建用户失败"))?;
        Ok(ApiResp::success(created))
    }

    pub async fn login_user2(&self, payload: &User2Login) -> AppResult<ApiResp<User2>> {
        let user = self
            .user2_repo()
            .get_by_name(&payload.user_name)
            .await?
            .ok_or_else(|| anyhow!("用户{}不存在", payload.user_name))?;
        if user.password != payload.password {
            return Ok(ApiResp::fail(format!("密码{}不正确", payload.password)));
        }
        let token = self.issue_token(user.user_id, &user.password)?;
        let mut with_token = user.clone();
        with_token.token = Some(token);
        Ok(ApiResp::success(with_token))
    }

    pub async fn register_user2(&self, mut u: User2) -> AppResult<ApiResp<String>> {
        if self.user2_repo().get_by_name(&u.user_name).await?.is_some() {
            return Ok(ApiResp::fail(format!("用户{}已存在", u.user_name)));
        }
        let next_id = self.user2_repo().list_all().await?.len() as i32 + 1;
        u.user_id = next_id;
        self.user2_repo().add(&u).await?;
        Ok(ApiResp::success_msg("注册成功"))
    }

    pub async fn list_user2(&self) -> AppResult<ApiResp<Vec<User2>>> {
        Ok(ApiResp::success(self.user2_repo().list_all().await?))
    }

    pub async fn delete_user2(&self, id: i32) -> AppResult<ApiResp<String>> {
        self.user2_repo().delete(id).await?;
        Ok(ApiResp::success_msg("删除成功"))
    }

    pub async fn update_user2(&self, u: User2) -> AppResult<ApiResp<Vec<User2>>> {
        self.user2_repo().update(&u).await?;
        Ok(ApiResp::success(self.user2_repo().list_all().await?))
    }

    pub async fn add_user2(&self, u: User2) -> AppResult<ApiResp<Vec<User2>>> {
        self.user2_repo().add(&u).await?;
        Ok(ApiResp::success(self.user2_repo().list_all().await?))
    }

    pub async fn search_user2(&self, keyword: &str) -> AppResult<ApiResp<Vec<User2>>> {
        Ok(ApiResp::success(self.user2_repo().search(keyword).await?))
    }

    pub async fn page_users(&self, q: &PageQuery) -> AppResult<ApiResp<PageResp<UserView>>> {
        Ok(ApiResp::success(self.user_repo().page_like(q).await?))
    }

    pub async fn select_all(&self) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(self.user_repo().list_all().await?))
    }

    pub async fn select_by_uid(&self, uid: i32) -> AppResult<ApiResp<Option<UserView>>> {
        Ok(ApiResp::success(self.user_repo().get_by_uid(uid).await?))
    }

    pub async fn select_by_uname(&self, uname: &str) -> AppResult<ApiResp<Option<UserView>>> {
        Ok(ApiResp::success(
            self.user_repo().get_by_uname(uname).await?,
        ))
    }

    pub async fn select_by_more(
        &self,
        uname: &str,
        cid: i32,
    ) -> AppResult<ApiResp<Option<UserView>>> {
        Ok(ApiResp::success(
            self.user_repo().get_by_more(uname, cid).await?,
        ))
    }

    pub async fn select_by_more_like(
        &self,
        uname: &str,
        cid: i32,
    ) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(
            self.user_repo().get_by_more_like(uname, cid).await?,
        ))
    }

    pub async fn select_by_deid(&self, deid: i32) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(self.user_repo().by_deid(deid).await?))
    }

    pub async fn select_by_gid(&self, gid: i32) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(self.user_repo().by_gid(gid).await?))
    }

    pub async fn select_by_branch(&self, bid: i32) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(self.user_repo().by_branch(bid).await?))
    }

    pub async fn select_branch_by_gid(&self, gid: i32) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(
            self.user_repo().branch_users_by_gid(gid).await?,
        ))
    }

    pub async fn select_auditor(&self) -> AppResult<ApiResp<Vec<UserView>>> {
        Ok(ApiResp::success(self.user_repo().auditors().await?))
    }

    pub async fn update_deid(&self, id: i32, deid: i32) -> AppResult<ApiResp<()>> {
        self.user_repo().update_deid(id, deid).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn update_avatar(&self, id: i32, avatar: &str) -> AppResult<ApiResp<()>> {
        self.user_repo().update_avatar(id, avatar).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn update_username(&self, id: i32, username: &str) -> AppResult<ApiResp<()>> {
        self.user_repo().update_username(id, username).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn update_account(&self, id: i32, uname: &str, psw: &str) -> AppResult<ApiResp<()>> {
        self.user_repo().update_account(id, uname, psw).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn delete_gid(&self, id: i32) -> AppResult<ApiResp<()>> {
        self.user_repo().delete_gid(id).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn set_gid(&self, ids: &[i32], gid: i32) -> AppResult<ApiResp<()>> {
        for id in ids {
            self.user_repo().update_gid(*id, gid).await?;
        }
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn add_user_view(&self, u: &UserUpsert) -> AppResult<ApiResp<()>> {
        self.user_repo().insert(u).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn update_user_view(&self, u: &UserUpsert) -> AppResult<ApiResp<()>> {
        self.user_repo().update(u).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn delete_user_view(&self, id: i32) -> AppResult<ApiResp<()>> {
        self.user_repo().delete(id).await?;
        Ok(ApiResp::success_msg("success"))
    }

    pub async fn delete_user_batch(&self, ids: &[i32]) -> AppResult<ApiResp<()>> {
        for id in ids {
            self.user_repo().delete(*id).await?;
        }
        Ok(ApiResp::success_msg("success"))
    }

    fn issue_token(&self, aud: i32, secret: &str) -> AppResult<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow!(e))?
            .as_secs()
            + 7 * 24 * 3600;
        let claims = JwtClaims {
            aud: aud.to_string(),
            exp: exp as usize,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| anyhow!(e))?;
        Ok(token)
    }
}
