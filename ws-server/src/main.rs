use axum::{Router, routing::get};
use tokio::sync::RwLock;
use std::{
    net::SocketAddr, 
};

use crate::{
    redis_manager::start_redis, socket_manager::ws_handler, subscription_manager::SubscriptionManager, user_manager::UserManager, 
};

mod redis_manager;
mod socket_manager;
mod subscription_manager;
mod types;
mod user;
mod user_manager;

#[derive(Clone)]
pub struct AppState {
    pub subscription_state : &'static RwLock<SubscriptionManager> ,
    pub user_state : &'static RwLock<UserManager>
}


#[tokio::main]
async fn main() {
    let markets = Vec::from( ["BTCUSDT".to_string() , "SOLUSDC".to_string() , "ETHUSDT".to_string()]);

    let subscription_state = SubscriptionManager::get_instance();
    let user_state = UserManager::get_instance();

    let app_state = AppState{
        subscription_state,
        user_state
    };

    // this is listening to the redis pub sub
    // we also have to pass it the names of the channels where it has to listen continuously!
    tokio::spawn(async move { start_redis(subscription_state , markets).await.unwrap() });


    // we have passed it the state for the subscription state! 
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let addr = SocketAddr::from((([127, 0, 0, 1]), 3001));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on ws://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}
