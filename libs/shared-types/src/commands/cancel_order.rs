use serde::{Deserialize, Serialize};

use crate::commands::create_order::PositionSide;


#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderCmd {
    request_id : String,
    user_id : i64,
    order_id :i64,
    market : String,
    price: i64,        // needed to find the price level in orderbook
    side : PositionSide
}

