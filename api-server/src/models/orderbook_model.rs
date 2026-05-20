use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

pub struct Bids {
    availableqty : i32 ,
    open_orders : BTreeMap < i32, Vec<Openorder>> // here i32 is price, so the data sorted as per price
}

pub struct Openorder {
    userid: i32 , 
    qty: i32,
    filled_qty: i32,
    order_id : i32,
    created_at: DateTime<Utc>,
}

pub struct  Orderbook {
    bids:  Bids,
    asks:  Bids, 
    last_traded_price: f32,
    index_price: f32
}
