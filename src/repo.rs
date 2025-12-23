use crate::error::AppResult;
use crate::model::UserDto;
use sqlx::PgPool;

pub struct UserRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    // 对应 APIs.md 的 /users 列表，按 Java 返回字段映射
    pub async fn list_users(&self) -> AppResult<Vec<UserDto>> {
        // 通过 COALESCE 保证非空，便于映射到非 Option 字段
        let users = sqlx::query_as::<_, UserDto>(
            r#"
            SELECT
                COALESCE(u.id, 0)                 AS user_id,
                COALESCE(u.uname, '')             AS user_name,
                COALESCE(u.psw, '')               AS password,
                COALESCE(u.points, 0)             AS points,
                COALESCE(u.cid, 0)                AS role_id,
                COALESCE(c.cname, '')             AS role_name,
                COALESCE(c.permissions, '')       AS role_permissions
            FROM djpt.user_view u
            LEFT JOIN djpt.chara_view c
              ON c.id = u.cid
            ORDER BY u.id
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(users)
    }
}
