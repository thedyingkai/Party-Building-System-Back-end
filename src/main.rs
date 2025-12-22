use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod error;
mod model;
mod repo;
mod service;
mod handler;

use handler::{routes, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 从环境变量读取连接串
    let database_url = std::env::var("DATABASE_URL")
        .expect("请设置环境变量 DATABASE_URL，例如：postgres://zyx:%40zyx20222110@119.3.218.22:26000/djpt");

    // 创建连接池（`sqlx 0.9-alpha` 无 `connect_timeout`）
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let state = AppState { pool };

    // 让类型自动推断，避免显式注解为 Router<AppState>
    let app = routes().with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    // `axum::serve` 返回 `Result<(), Infallible>`
    axum::serve(listener, app).await.expect("服务器运行失败");

    Ok(())
}