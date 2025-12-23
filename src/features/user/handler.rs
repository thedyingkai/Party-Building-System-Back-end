use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::error::AppError;
use crate::handler::AppState;

use super::models::{ApiResp, PageQuery, User2, User2Login, UserUpsert, UserView};
use super::service::UserService;

#[derive(Deserialize)]
struct UnameCid {
    uname: String,
    cid: i32,
}

#[derive(Deserialize)]
struct LoginUserPayload {
    uname: String,
    psw: String,
}

#[derive(Deserialize)]
struct UpdateDeidPayload {
    id: i32,
    deid: i32,
}

#[derive(Deserialize)]
struct UpdateAvatarPayload {
    id: i32,
    avatar: String,
}

#[derive(Deserialize)]
struct UpdateUsernamePayload {
    id: i32,
    username: String,
}

#[derive(Deserialize)]
struct UpdateAccountPayload {
    id: i32,
    uname: String,
    psw: String,
}

#[derive(Deserialize)]
struct SetGidQuery {
    gid: i32,
}

fn svc(pool: &PgPool) -> UserService {
    UserService::new(pool.clone())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello))
        .route("/index", get(index))
        .route("/login", post(login_user))
        .route("/register", post(register_user))
        .route("/log_in", post(login_user2))
        .route("/sign_in", post(sign_up_user2))
        .route("/token/create", post(create_token))
        .route("/token/get_user", get(get_current_user))
        .route("/users", get(list_user2))
        .route("/users/delete/:u_id", get(delete_user2))
        .route("/users/update", post(update_user2))
        .route("/users/add", post(add_user2))
        .route("/users/search/:keyword", get(search_user2))
        .route("/user/add", post(user_add))
        .route("/user/update", put(user_update))
        .route("/user/delete/:uid", delete(user_delete))
        .route("/user/delete/batch", delete(user_delete_batch))
        .route("/user/selectAll", get(user_select_all))
        .route("/user/selectByUid/:uid", get(user_select_by_uid))
        .route("/user/selectByUname/:uname", get(user_select_by_uname))
        .route("/user/selectByMore", get(user_select_by_more))
        .route("/user/selectByMoreLike", get(user_select_by_more_like))
        .route("/user/selectByPage", get(user_select_by_page))
        .route("/user/selectByDeid/:deid", get(user_select_by_deid))
        .route("/user/selectByGid/:gid", get(user_select_by_gid))
        .route("/user/selectByBranch/:bid", get(user_select_by_branch))
        .route(
            "/user/selectBranchByGid/:gid",
            get(user_select_branch_by_gid),
        )
        .route("/user/selectAuditor", get(user_select_auditor))
        .route("/user/updateDeid", put(user_update_deid))
        .route("/user/updateAvatar", put(user_update_avatar))
        .route("/user/updateUsername", put(user_update_username))
        .route("/user/updateAccount", put(user_update_account))
        .route("/user/deletegid/:id", put(user_delete_gid))
        .route("/user/setgid", put(user_set_gid))
}

async fn hello(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).hello().await?))
}

async fn index(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).index().await?))
}

async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool)
            .login_user(&payload.uname, &payload.psw)
            .await?,
    ))
}

async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<UserUpsert>,
) -> Result<impl IntoResponse, AppError> {
    if payload.uname.len() > 10 || payload.psw.len() > 20 {
        return Ok(Json(ApiResp::<UserView>::fail("用户名或密码长度非法")));
    }
    Ok(Json(svc(&state.pool).register_user(&payload).await?))
}

async fn login_user2(
    State(state): State<AppState>,
    Json(payload): Json<User2Login>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).login_user2(&payload).await?))
}

async fn sign_up_user2(
    State(state): State<AppState>,
    Json(payload): Json<User2>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).register_user2(payload).await?))
}

async fn create_token(
    State(state): State<AppState>,
    Json(user): Json<User2>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool)
            .login_user2(&User2Login {
                user_name: user.user_name.clone(),
                password: user.password.clone(),
            })
            .await?,
    ))
}

async fn get_current_user() -> Result<impl IntoResponse, AppError> {
    Ok(Json(ApiResp::<String>::fail(
        "token-based lookup not implemented; integrate with middleware",
    )))
}

async fn list_user2(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).list_user2().await?))
}

async fn delete_user2(
    State(state): State<AppState>,
    Path(u_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).delete_user2(u_id).await?))
}

async fn update_user2(
    State(state): State<AppState>,
    Json(u): Json<User2>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).update_user2(u).await?))
}

async fn add_user2(
    State(state): State<AppState>,
    Json(u): Json<User2>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).add_user2(u).await?))
}

async fn search_user2(
    State(state): State<AppState>,
    Path(keyword): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).search_user2(&keyword).await?))
}

async fn user_add(
    State(state): State<AppState>,
    Json(u): Json<UserUpsert>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).add_user_view(&u).await?))
}

async fn user_update(
    State(state): State<AppState>,
    Json(u): Json<UserUpsert>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).update_user_view(&u).await?))
}

async fn user_delete(
    State(state): State<AppState>,
    Path(uid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).delete_user_view(uid).await?))
}

async fn user_delete_batch(
    State(state): State<AppState>,
    Json(ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).delete_user_batch(&ids).await?))
}

async fn user_select_all(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_all().await?))
}

async fn user_select_by_uid(
    State(state): State<AppState>,
    Path(uid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_by_uid(uid).await?))
}

async fn user_select_by_uname(
    State(state): State<AppState>,
    Path(uname): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_by_uname(&uname).await?))
}

async fn user_select_by_more(
    State(state): State<AppState>,
    Query(q): Query<UnameCid>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool).select_by_more(&q.uname, q.cid).await?,
    ))
}

async fn user_select_by_more_like(
    State(state): State<AppState>,
    Query(q): Query<UnameCid>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool)
            .select_by_more_like(&q.uname, q.cid)
            .await?,
    ))
}

async fn user_select_by_page(
    State(state): State<AppState>,
    Query(q): Query<PageQuery>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).page_users(&q).await?))
}

async fn user_select_by_deid(
    State(state): State<AppState>,
    Path(deid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_by_deid(deid).await?))
}

async fn user_select_by_gid(
    State(state): State<AppState>,
    Path(gid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_by_gid(gid).await?))
}

async fn user_select_by_branch(
    State(state): State<AppState>,
    Path(bid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_by_branch(bid).await?))
}

async fn user_select_branch_by_gid(
    State(state): State<AppState>,
    Path(gid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_branch_by_gid(gid).await?))
}

async fn user_select_auditor(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).select_auditor().await?))
}

async fn user_update_deid(
    State(state): State<AppState>,
    Json(p): Json<UpdateDeidPayload>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).update_deid(p.id, p.deid).await?))
}

async fn user_update_avatar(
    State(state): State<AppState>,
    Json(p): Json<UpdateAvatarPayload>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).update_avatar(p.id, &p.avatar).await?))
}

async fn user_update_username(
    State(state): State<AppState>,
    Json(p): Json<UpdateUsernamePayload>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool).update_username(p.id, &p.username).await?,
    ))
}

async fn user_update_account(
    State(state): State<AppState>,
    Json(p): Json<UpdateAccountPayload>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        svc(&state.pool)
            .update_account(p.id, &p.uname, &p.psw)
            .await?,
    ))
}

async fn user_delete_gid(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).delete_gid(id).await?))
}

async fn user_set_gid(
    State(state): State<AppState>,
    Query(q): Query<SetGidQuery>,
    Json(ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(svc(&state.pool).set_gid(&ids, q.gid).await?))
}
