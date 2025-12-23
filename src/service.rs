use crate::error::AppResult;
use crate::model::UserDto;
use crate::repo::UserRepo;
use sqlx::PgPool;

pub struct UserService<'a> {
    repo: UserRepo<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self {
            repo: UserRepo::new(pool),
        }
    }

    pub async fn list_users(&self) -> AppResult<Vec<UserDto>> {
        self.repo.list_users().await
    }
}
