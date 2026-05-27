use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderCommand {
    pub user_id: i64,
    pub market: String,
    pub side: OrderSide,
    pub price: i64,
    pub qty: i64,
    pub margin: i64
}


#[derive(Debug,Clone,PartialEq,   Serialize, Deserialize)]
pub enum OrderSide {
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