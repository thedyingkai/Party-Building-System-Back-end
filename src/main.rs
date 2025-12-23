use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod common;
mod error;
mod features;
mod handler;
mod middleware;
mod model;
mod repo;
mod service;

use crate::middleware::jwt_auth;
use axum::middleware::from_fn_with_state;
use handler::{AppState, routes};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").expect("请设置环境变量 DATABASE_URL，例如：postgres://...");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let state = AppState { pool };

    let app = routes()
        .layer(from_fn_with_state(state.pool.clone(), jwt_auth::jwt_auth))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    axum::serve(listener, app).await.expect("服务器运行失败");

    Ok(())
}
