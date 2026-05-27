use chrono::{DateTime, Utc};

pub struct Fill {
    pub fill_id: i64,

    pub maker_user_id: i32,

    pub taker_user_id: i32,

    pub market: String,

    pub qty: i64,

    pub price: i64,

    pub maker_order_id: i64,

    pub taker_order_id: i64,

    pub created_at: DateTime<Utc>,
}
