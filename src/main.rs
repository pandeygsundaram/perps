mod claims;
mod auth;

use std::sync::Mutex;

// use std::{collections::BTreeMap, sync::Mutex};
// use chrono::{DateTime, Utc};
use axum::{ Router, middleware,  routing::{delete, get, post}};

use crate::auth::{User, login_handler, signup_handler};
use crate::claims::auth_middleware;


pub static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());


// struct OnrampReturnType {
//     message: String,
// }


// fn add_balance(userid : i32, addamount : i32) -> (StatusCode, String) {
//     let mut users = USERS.lock().unwrap();
//     let  user = users.iter_mut().find(|u| u.id == userid);
//     match user {
//         Some(u)=> {
//             u.balance += addamount;
//             return (StatusCode::OK , "User updated successfully".to_string());

//         },
//         _ => {
//             return (StatusCode::NOT_FOUND, "User not found".to_string() );

//         }
        
//     } 
// }


#[tokio::main]
async fn main() {
    

    // these routes require a valid JWT
    let protected = Router::new()
        .route("/onramp", post(|| async { "Sends me more money bitch" }))
        .route("/order", post(|| async { "Create order endpoint" }))
        .route("/order", delete(|| async { "Delete order endpoint" }))
        .route("/equity/available", get(|| async { "Equity endpoint" }))
        .route("/positions/open/{marketId}", get(|| async { "Open positions" }))
        .route("/positions/closed/{marketId}", get(|| async { "Closed positions" }))
        .route("/orders/open/{marketId}", get(|| async { "Open orders" }))
        .route("/orders/{marketId}", get(|| async { "Orders by market" }))
        .route("/fills", get(|| async { "Fills endpoint" }))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
        .merge(protected);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}




// struct Bids {
//     availableqty : i32 ,
//     open_orders : BTreeMap < i32, Vec<Openorder>> // here i32 is price, so the data sorted as per price
// }

// struct Openorder {
//     userid: i32 , 
//     qty: i32,
//     filled_qty: i32,
//     order_id : i32,
//     created_at: DateTime<Utc>,
// }

// struct  Orderbook {
//     bids:  Bids,
//     asks:  Bids, 
//     last_traded_price: f32,
//     index_price: f32
// }


// okay now we go ahead here and basically create these endpoints 



