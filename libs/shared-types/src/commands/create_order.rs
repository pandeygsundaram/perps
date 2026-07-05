use serde::{Deserialize, Serialize};

#[derive(Debug, Clone , Serialize, Deserialize)]
pub struct CreateOrderCommand {
    pub user_id: i64,
    pub market: String,
    pub side: PositionSide,
    pub price: i64,
    pub qty: i64,
    pub margin: i64
}


#[derive(Debug,Clone,PartialEq,   Serialize, Deserialize)]
pub enum PositionSide {
    LONG,
    SHORT,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    LIMIT,
    MARKET,

}


pub struct CancelOrder{
    user_id : i64,
    market_id:i64,
    order_id:i64,

}

pub struct ClosePosition{
    position_id:i64,
    user_id:i64,


}
pub struct AddMarket {
    market_id:i64,
    market_name:String
}



// // POST /markets  →  AddMarket  (admin only)
// struct AddMarketCmd {
//     string   request_id;
//     string   market_id;        // e.g. "BTC"
//     string   name;             // e.g. "Bitcoin"
//     int64_t  initial_price;    // starting index price
//     double   maker_fee;        // e.g. 0.0002 = 0.02%
//     double   taker_fee;        // e.g. 0.0005 = 0.05%
//     double   maintenance_margin_rate;  // e.g. 0.05 = 5%
//     double   initial_margin_rate;      // e.g. 0.10 = 10% (1/leverage)
// };

// ============================================================
// ENGINE ACKS
// ============================================================
// Published to `engine:acks` pubsub after each command is processed.
// API server subscribes and matches by request_id to unblock the
// waiting HTTP handler.
//
//   PUBLISH engine:acks  {"request_id":"...", "status":"ok", "data":{...}}
// ============================================================

// enum class AckStatus { OK, ERROR };

// struct EngineAck {
//     string    request_id;
//     AckStatus status;
//     string    error_message;   // empty if OK
//     // response payload varies by command — serialised as JSON string in real engine
//     // e.g. for CreateOrder: {"order_id": 42}
//     // e.g. for AddBalance:  {"available": 900, "locked": 100}
// };
