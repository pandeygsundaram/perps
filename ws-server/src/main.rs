use axum::{Router,  routing::get};
use std::net::SocketAddr;

use crate::{redis_manager::start_redis, socket_manager::ws_handler};

mod redis_manager;
mod socket_manager;
mod subscription_manager;
mod types;
mod user;
mod user_manager;

#[tokio::main]
async fn main() {

    tokio::spawn(async { start_redis().await.unwrap() });

    let app = Router::new().route("/ws", get(ws_handler));

    let addr = SocketAddr::from((([127, 0, 0, 1]), 3001));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on ws://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}

