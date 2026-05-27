use shared_types::commands::create_order::{OrderSide, };

use crate::models::{balance::Balance, position::Position};


//order type will be added later, for now only focusing on limit order flow

pub enum OrderCommand {
    CreateOrder{
        order_id: i64,
        user_id: i64,
        side: OrderSide,
        price:i64,
        qty:i64,
        margin:i64,

    },
    CancelOrder{
        order_id:i64,
        user_id: i64
    },
    Addbalance{
        user_id : i64,
        amount : i64,
    },
    AddMarket {
        market: String
    },
    CloseOrder{
        user_id: i64,
        order_id: i64,
        market_id: i64
    },
    GetPosition{
        market_id : i64,
        user_id: i64
    },
//    GetBalance {
//        user_id: i64
//    },
//    // this via wss
//  GetOpenOrder {
//        user_id: i64,
//        market_id: i64,
//    },
    //wss
//    GetAllOrderInMarket  {
//        market_id: i64
//    },

}

pub enum OrderResponse {
    CreateOrderResponse{
        // ack 
        order_id : i64,
    },
    CancelOrderResponse{
        msg : String
    },
    AddBalanceResponse{
        user_id : i64,
        final_balance: Balance
    },
    AddMarketResponse {
        msg : String
    },
    CloseOrderResponse {
        msg : String
    },
    GetPositionResponse{
        data: Position
    },
    GetBalanceResponse{
        data: Balance
    },
    GetOpenOrderResponse{


    },




}