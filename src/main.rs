use std::{collections::BTreeMap, string, sync::Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use axum::{
    Json, Router, http::StatusCode, routing::{get, post}
};

static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());


#[derive(Clone , Serialize , Deserialize)]
struct User {
    id: i32,
    username: String,
    name: String,
    balance : i32,
    password: String,
}

#[derive(Deserialize)]
struct  UserDetails{
    username:String,
    name: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginCredential {
    username: String,
    password: String
}

#[derive(Serialize)]
struct LoginReturnType {
    message: String,
    data : Option<User> 
}



fn login( cred: LoginCredential) -> Option<User> {
    let users  = USERS.lock().unwrap();
    let user = users.iter().find(|u| u.username == cred.username);

    match user {
        Some(u)=> {
            print!("User found, let's match password");
            if u.password == cred.password {
                Some(u.clone())
            }else {
                return None;
            }
        },
        _ =>{
            print!("User not found bitch");
            return None;
        }
    }

}

fn signup (user_details: UserDetails) {
    let mut users = USERS.lock().unwrap();

    let last_user = users.last();
    let mut last_id = 0;
    match last_user {
        Some(user) =>{
            println!("{}", user.id);
            last_id = user.id;
        },
        _  => {
            print!("Do nothing");
        }
    }
    let newuser = User {
        id : last_id+1,
        name : user_details.name,
        password : user_details.password,
        balance : 0,
        username : user_details.username

    };

    users.push(newuser);

}


// fn getme(username :String) -> Option<User> {

//     let users = USERS.lock().unwrap();
//     let user = users.iter().find(|u| u.username == username );
//     match user {
//         Some(u)=>{
//             return Some(u.clone());
//         },
//         None =>{
//             return None;
//         }  
//     };

// }


async fn signup_handler (Json(user_details) : Json<UserDetails> ) -> String {
    signup(user_details);

    "User created".to_string()

}




async fn login_handler( Json(cred) : Json<LoginCredential>) ->(StatusCode, Json<LoginReturnType>) {

    match  login(cred) {
        Some(u) =>{
            println!("{}",u.name);
            let responce = LoginReturnType {
                message: "Loggedin Successfully".to_string(),
                data: Some(u)
            };
            return (StatusCode::OK , Json(responce)); 
        },
        _ => {
            let responce = LoginReturnType{
                message: "Invalid Credentials".to_string(),
                data : None,
            };
            return (StatusCode::UNAUTHORIZED, Json(responce));
        }   
    }
}




#[tokio::main]
async fn main() {
    

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/signup", post(signup_handler))
    .route("/login", post(login_handler));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    pri();
    axum::serve(listener, app).await.unwrap();


}




struct Bids {
    availableqty : i32 ,
    open_orders : BTreeMap < i32, Vec<Openorder>> // here i32 is price, so the data sorted as per price
}

struct Openorder {
    userid: i32 , 
    qty: i32,
    filled_qty: i32,
    order_id : i32,
    created_at: DateTime<Utc>,
}

struct  Orderbook {
    bids:  Bids,
    asks:  Bids, 
    last_traded_price: f32,
    index_price: f32
}



















// fn main() {
//     println!("Hello, world!");
//     pri();
// }




///so basically what i have to do is that
/// create a server which receives a bunch of requests and allows the user to send market orders of various type and do a bunch of thingy
/// so the way i need to start is create a api server with auth first
/// then there is going to be a bunch of things for orderbook and positions and fills and balances
/// and there are going to be other things i am assuming so talking about the req which i am sending is that 
/// auth endpoint is going to be there
/// create order
/// cancel order
/// get positions 
/// abhi ke liye i can make everything in a single file 
/// let's say there is going to be a orderbook array in here only and after that i can take it to better architecture
/// and what's next?? the next thing to do is what is the schema of orderbook???
/// what else ?
/// umm so a array of balances() [mother process], orderbook ( ) [individual threads], array of positions [part of mother process only ig] ,
/// so basically i will also have 
/// 
/// 
/// so basically i am buying a bunch of quantity(long )
/// 
/// 
/// $50 * 10 quantity -> 10x leverage
/// $500 => $100(sol perp)
/// sol perp i have?? 5 sol perp
/// 
///      -> sol per went to 150  
/// 150*5= $ 750
/// so the amount i have is now 750 ,
/// so the thing here is that my original capital was 50 and 10 quantity, so i will basically get $ 75,
/// 
/// okay so apparently this is spot logic and not the logic of perpetuals !!
/// for perpetuals i think it will be like this 
/// 
/// i opened my position here
/// { $50 colateral, quantity : 10 , dirn : long} , solperp_price: $100
/// leverage is 20x
/// 
/// 1 -> 0
/// 1-> 50
/// 500
/// now price is $ 150
/// 
/// 
/// 
/// 
fn pri(){
    println!("Yp");
}




// so go ahead aand build the auth logic going on from here on and after that you can basically do other things


// point is i don't know how to write the auth and all in here so iwll ahve to learn that



//import express from "express";

// const app = express();
// app.use(express.json());

// const users = [{
//     userId: 1,
//     username: "harkirat",
//     password: 123123,
//     collateral: {
//          availabe: 2000,
//          locked: 1000
//     },
//      positions: [
//         { market: "SOL", type: "LONG", qty: 10, margin: 500, liquidationPrice: 80, averagePrice: 90 },
//         { market: "ETH", type: "SHORT", qty: 1, margin: 500, liquidationPrice: 2000, averagePrice: 1900 }
//     ],
//     orders: [
//         { orderId: 1, market: "SOL", type: "LONG", qty: 10, margin: 500, orderType: "limit", price: 90, status: "filled" },
//         { orderId: 2, market: "ETH", type: "SHORT", qty: 10, margin: 500, orderType: "limit", price: 1900, status: "filled" },
//         { orderId: 3, market: "BTC", type: "LONG", qty: 10, margin: 500, orderType: "limit", price: 1900, status: "cancelled" },
//     ]
// }, {
//     userId: 2,
//     username: "raman",
//     password: 123123,
//     collateral: {
//          availabe: 2000,
//          locked: 2000
//     },
//     positions: [
//         { market: "SOL", type: "SHORT", qty: 10,  margin: 1000, liquidationPrice: 80, pnL: 200, averagePrice: 90 },
//         { market: "ETH", type: "LONG", qty: 1, margin: 1000, liquidationPrice: 2000, pnL: -100, averagePrice: 1900 }
//     ],
//     orders: [
//         { orderId: 10, market: "SOL", type: "SHORT", qty: 10, margin: 500, orderType: "market", price: 90, status: "filled" },
//         { orderId: 11, market: "ETH", type: "LONG", qty: 10, margin: 500, orderType: "market", price: 1900, status: "filled" },
//         { orderId: 12, market: "ZEC", type: "LONG", qty: 10, margin: 500, orderType: "limit", price: 1900, status: "open" },
//     ]
// }];

// type Bid = {
//     availableQty: number,
//     openOrders: { userId: number, qty: number, filledQty: number, orderId: number, createdAt: Date }[]
// }

// type Orderbook = {
//     bids: Record<string, Bid>,
//     asks: Record<string, Bid>,
//     lastTradedPrice: number,
//     indexPrice: number
// }

// type Orderbooks = Record<string, Orderbook>

// const orderbooks: Orderbooks = {
//      SOL: { bids: {}, asks: {}, lastTradedPrice: 90, indexPrice: 90.01 },
//      ETH: { bids: {}, asks: {}, lastTradedPrice: 1900, indexPrice: 1899.9 }
// }

// const fills = [{
//     maker: 1,
//     taker: 2,
//     market: "SOL",
//     qty: 10,
//     price: 90,
//     long: 1,
//     short: 2
// }, {
//     maker: 1,
//     taker: 2,
//     market: "ETH",
//     qty: 1,
//     price: 1900,
//     long: 2,
//     short: 1
// }];

// app.post("/signup", (req, res) => {})
// app.post("/signin", (req, res) => {})
// app.post("/onramp", (req, res) => {})
// app.post("/order", (req, res) => {})
// app.delete("/order", (req, res) => {})
// app.get("/equity/available", (req, res) => {})
// app.get("/positions/open/:marketId", (req, res) => {});
// app.get("/positions/closed/:marketId", (req, res) => {});
// app.get("/orders/open/:marketId", (req, res) => {})
// app.get("/orders/:marketId", (req, res) => {})
// app.get("/fills", (req, res) => {});

// async function liqudationChecks(asset: string, price: number) {

// }


// async function onPriceUpdateFromBinance(asset: string, price: number) {
//     liqudationChecks(asset, price);   
// }
