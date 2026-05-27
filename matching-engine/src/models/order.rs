use chrono::{DateTime, Utc};
use shared_types::commands::create_order::OrderSide;



pub enum OrderStatus {
    OPEN,
    FILLED,
    CANCELLED,
    PARTIALLYFILLED,
}


// for now only limit order we will update that later
pub struct Order {
    pub order_id: i64,
    pub user_id: i64,
    pub market: String,
    pub side: OrderSide,
    // pub order_type: OrderType,
    pub qty: i64,
    pub filled_qty: i64,
    pub price: i64,
    pub margin: i64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}