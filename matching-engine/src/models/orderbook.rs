use std::collections::BTreeMap;

use chrono::{
    DateTime,
    Utc,
};

#[derive(Debug, Clone)]
pub struct OpenOrder {

    pub user_id: i64,
    pub qty: i64,
    pub filled_qty: i64,
    pub order_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PriceLevel {
    pub total_qty: i64,
    pub orders: Vec<OpenOrder>,
}

#[derive(Debug, Clone)]
pub struct Orderbook {
    // price -> orders
    pub bids: BTreeMap<i64, PriceLevel>,
    // price -> orders
    pub asks: BTreeMap<i64, PriceLevel>,
    pub last_traded_price: i64,
    pub index_price: i64,
}