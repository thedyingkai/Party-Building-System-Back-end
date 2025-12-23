use sqlx::{PgPool, Row};

use crate::error::AppResult;

use super::models::{PageQuery, PageResp, User2, UserUpsert, UserView};

pub struct User2Repo<'a> {
    pool: &'a PgPool,
}

impl<'a> User2Repo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_all(&self) -> AppResult<Vec<User2>> {
        let rows = sqlx::query_as::<_, User2>(
            r#"
            SELECT user_id, user_name, password, points, user2.role_id, role_name, role_permissions,
                   NULL::text AS token
            FROM User2
            JOIN Role ON User2.role_id = Role.role_id
            ORDER BY user_id ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_by_name(&self, name: &str) -> AppResult<Option<User2>> {
        let row = sqlx::query_as::<_, User2>(
            r#"
            SELECT user_id, user_name, password, points, user2.role_id, role_name, role_permissions,
                   NULL::text AS token
            FROM User2
            JOIN Role ON User2.role_id = Role.role_id
            WHERE user_name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(self.pool)
        .await?;
        Ok(row)
    }

    pub async fn get_by_id(&self, id: i32) -> AppResult<Option<User2>> {
        let row = sqlx::query_as::<_, User2>(
            r#"
            SELECT user_id, user_name, password, points, user2.role_id, role_name, role_permissions,
                   NULL::text AS token
            FROM User2
            JOIN Role ON User2.role_id = Role.role_id
            WHERE user_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(row)
    }

    pub async fn add(&self, u: &User2) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO User2(user_id, user_name, password, points, role_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(u.user_id)
        .bind(&u.user_name)
        .bind(&u.password)
        .bind(u.points)
        .bind(u.role_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn update(&self, u: &User2) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE User2
            SET user_name = $1, password = $2, points = $3, role_id = $4
            WHERE user_id = $5
            "#,
        )
        .bind(&u.user_name)
        .bind(&u.password)
        .bind(u.points)
        .bind(u.role_id)
        .bind(u.user_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query("DELETE FROM User2 WHERE user_id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn search(&self, keyword: &str) -> AppResult<Vec<User2>> {
        let like = format!("%{}%", keyword);
        let rows = sqlx::query_as::<_, User2>(
            r#"
            SELECT user_id, user_name, password, points, user2.role_id, role_name, role_permissions,
                   NULL::text AS token
            FROM User2
            JOIN Role ON User2.role_id = Role.role_id
            WHERE user_name LIKE $1
            ORDER BY user_id ASC
            "#,
        )
        .bind(like)
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }
}

pub struct UserViewRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> UserViewRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, u: &UserUpsert) -> AppResult<()> {
        sqlx::query("INSERT INTO user_view (uname, psw, cid) VALUES ($1, $2, $3)")
            .bind(&u.uname)
            .bind(&u.psw)
            .bind(u.cid)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn update(&self, u: &UserUpsert) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE user_view
            SET cid = $1, uname = $2, psw = $3, points = $4, gid = $5, bid = $6, deid = $7,
                username = $8, avatar = $9
            WHERE id = $10
            "#,
        )
        .bind(u.cid)
        .bind(&u.uname)
        .bind(&u.psw)
        .bind(u.points)
        .bind(u.gid)
        .bind(u.bid)
        .bind(u.deid)
        .bind(&u.username)
        .bind(&u.avatar)
        .bind(u.id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i32) -> AppResult<()> {
        sqlx::query("DELETE FROM user_view WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_all(&self) -> AppResult<Vec<UserView>> {
        let rows = sqlx::query_as::<_, UserView>(
            r#"
            SELECT u.id, u.cid, u.gid, u.bid, u.deid, u.uname, u.psw, u.points,
                   u.username, u.avatar, c.permissions,
                   c.cname,
                   NULL::text AS branch_name,
                   NULL::text AS group_name,
                   NULL::text AS depart_name,
                   NULL::text AS sector_name
            FROM user_view u
            JOIN chara c ON u.cid = c.cid
            ORDER BY u.id
            "#,
        )
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_by_uid(&self, uid: i32) -> AppResult<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(
            r#"
            SELECT u.id, u.cid, u.gid, u.bid, u.deid, u.uname, u.psw, u.points,
                   u.username, u.avatar, c.permissions,
                   c.cname,
                   NULL::text AS branch_name,
                   NULL::text AS group_name,
                   NULL::text AS depart_name,
                   NULL::text AS sector_name
            FROM user_view u
            JOIN chara c ON u.cid = c.cid
            WHERE u.id = $1
            "#,
        )
        .bind(uid)
        .fetch_optional(self.pool)
        .await?;
        Ok(row)
    }

    pub async fn get_by_uname(&self, uname: &str) -> AppResult<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(
            r#"
            SELECT user_view.id,
                   user_view.cid,
                   user_view.gid,
                   group_view.bid,
                   user_view.deid,
                   user_view.uname,
                   user_view.psw,
                   user_view.points,
                   user_view.username,
                   user_view.avatar,
                   chara.permissions,
                   chara.cname,
                   branch_view.name AS branch_name,
                   group_view.name AS group_name,
                   department_view.name AS depart_name,
                   sector_view.name AS sector_name
            FROM user_view
            LEFT JOIN chara ON user_view.cid = chara.cid
            LEFT JOIN group_view ON group_view.gid = user_view.gid
            LEFT JOIN branch_view ON branch_view.bid = group_view.bid
            LEFT JOIN department_view ON department_view.id = user_view.deid
            LEFT JOIN sector_view ON sector_view.id = department_view.seid
            WHERE user_view.uname = $1
            "#,
        )
        .bind(uname)
        .fetch_optional(self.pool)
        .await?;
        Ok(row)
    }

    pub async fn get_by_more(&self, uname: &str, cid: i32) -> AppResult<Option<UserView>> {
        let row =
            sqlx::query_as::<_, UserView>("SELECT * FROM user_view WHERE uname = $1 AND cid = $2")
                .bind(uname)
                .bind(cid)
                .fetch_optional(self.pool)
                .await?;
        Ok(row)
    }

    pub async fn get_by_more_like(&self, uname: &str, cid: i32) -> AppResult<Vec<UserView>> {
        let like = format!("%{}%", uname);
        let rows = sqlx::query_as::<_, UserView>(
            "SELECT * FROM user_view WHERE uname LIKE $1 AND cid = $2 ORDER BY id ASC",
        )
        .bind(like)
        .bind(cid)
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn page_like(&self, q: &PageQuery) -> AppResult<PageResp<UserView>> {
        let uname_like = q
            .uname
            .as_deref()
            .map(|u| format!("%{}%", u))
            .unwrap_or_else(|| "%".to_string());
        let cid = q.cid.unwrap_or(0);
        let offset = (q.page_num.saturating_sub(1) * q.page_size) as i64;
        let list = sqlx::query_as::<_, UserView>(
            r#"
            SELECT * FROM user_view
            WHERE uname LIKE $1 AND cid = $2
            ORDER BY id ASC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(&uname_like)
        .bind(cid)
        .bind(q.page_size)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 =
            sqlx::query("SELECT COUNT(id) FROM user_view WHERE uname LIKE $1 AND cid = $2")
                .bind(&uname_like)
                .bind(cid)
                .fetch_one(self.pool)
                .await?
                .get(0);

        Ok(PageResp {
            total,
            page_num: q.page_num,
            page_size: q.page_size,
            list,
        })
    }

    pub async fn by_deid(&self, deid: i32) -> AppResult<Vec<UserView>> {
        sqlx::query_as::<_, UserView>("SELECT * FROM user_view WHERE deid = $1")
            .bind(deid)
            .fetch_all(self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn by_gid(&self, gid: i32) -> AppResult<Vec<UserView>> {
        sqlx::query_as::<_, UserView>("SELECT * FROM user_view WHERE gid = $1")
            .bind(gid)
            .fetch_all(self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn by_branch(&self, bid: i32) -> AppResult<Vec<UserView>> {
        sqlx::query_as::<_, UserView>(
            r#"
            SELECT user_view.* FROM user_view
            JOIN group_view ON user_view.gid = group_view.gid
            JOIN branch_view ON group_view.bid = branch_view.bid
            WHERE branch_view.bid = $1
            "#,
        )
        .bind(bid)
        .fetch_all(self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn auditors(&self) -> AppResult<Vec<UserView>> {
        sqlx::query_as::<_, UserView>(
            r#"
            SELECT user_view.* FROM user_view, chara_view
            WHERE user_view.cid = chara_view.id
              AND SUBSTRING(permissions FROM 3 FOR 1) = '1'
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn update_deid(&self, id: i32, deid: i32) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET deid = $1 WHERE id = $2")
            .bind(deid)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_gid(&self, id: i32, gid: i32) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET gid = $1 WHERE id = $2")
            .bind(gid)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_gid(&self, id: i32) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET gid = -1 WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_avatar(&self, id: i32, avatar: &str) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET avatar = $1 WHERE id = $2")
            .bind(avatar)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_username(&self, id: i32, username: &str) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET username = $1 WHERE id = $2")
            .bind(username)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_account(&self, id: i32, uname: &str, psw: &str) -> AppResult<()> {
        sqlx::query("UPDATE user_view SET uname = $1, psw = $2 WHERE id = $3")
            .bind(uname)
            .bind(psw)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn branch_users_by_gid(&self, gid: i32) -> AppResult<Vec<UserView>> {
        sqlx::query_as::<_, UserView>(
            r#"
            SELECT user_view.* FROM user_view, group_view, branch_view
            WHERE user_view.gid = group_view.gid
              AND group_view.bid = branch_view.bid
              AND branch_view.bid = (SELECT bid FROM group_view WHERE gid = $1)
            "#,
        )
        .bind(gid)
        .fetch_all(self.pool)
        .await
        .map_err(Into::into)
    }
}
