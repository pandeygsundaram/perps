mod claims;
mod routes;
mod handler;
mod dto;
mod models;
mod service;
mod utils;


use std::sync::Mutex;
use axum::Router;
use crate::{models::user_model::User, routes::app_routes, utils::db::connect_db};


pub static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = connect_db().await;

    

    let app = Router::new()
        .merge(app_routes())
        .with_state(db);

        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}



// ok so the thing is that so far i have done a bunch of things in here
// now the most logical step is to create the redis stream thingy so that somehow the add balance thingy works out
