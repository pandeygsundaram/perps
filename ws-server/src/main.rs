use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::sync::RwLock;

use crate::{
    redis_manager::start_redis, socket_manager::ws_handler,
    subscription_manager::SubscriptionManager, user_manager::UserManager,
};

mod redis_manager;
mod socket_manager;
mod subscription_manager;
mod types;
mod user_manager;

#[derive(Clone)]
pub struct AppState {
    pub subscription_state: &'static RwLock<SubscriptionManager>,
    pub user_state: &'static RwLock<UserManager>,
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);

    let markets: Vec<String> = std::env::var("MARKETS")
        .map(|m| {
            m.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_else(|_| {
            vec![
                "BTCUSDT".to_string(),
                "SOLUSDC".to_string(),
                "ETHUSDT".to_string(),
            ]
        });

        
    let subscription_state = SubscriptionManager::get_instance();
    let user_state = UserManager::get_instance();

    let app_state = AppState {
        subscription_state,
        user_state,
    };

    // this is listening to the redis pub sub
    // we also have to pass it the names of the channels where it has to listen continuously!
    tokio::spawn(async move { start_redis(markets).await.unwrap() });

    // we have passed it the state for the subscription state!
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let addr = SocketAddr::from((([127, 0, 0, 1]), port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on ws://127.0.0.1:{port}");

    axum::serve(listener, app).await.unwrap();
}
