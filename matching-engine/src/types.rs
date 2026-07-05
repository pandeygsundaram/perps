use std::collections::HashMap;

use redis::Value;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

// first mpsc type
#[derive(Debug)]
pub struct DispatcherToSettlementChannelProp {
    pub data: HashMap<String, Value>,
    pub reply: Sender<HashMap<String, Value>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Long,
    Short,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum EngineCommands {
    AddBalance {
        request_id: String,
        user_id: String,
        amount: i64,
    },
    WithdrawBalance {
        request_id: String,
        user_id: String,
        amount: i64,
    },
    GetBalance {
        request_id: String,
        user_id: String,
    },
    CreateOrder {
        request_id: String,
        user_id: String,
        qty: i64,
        side: Side,
        market: String,
        order_type: OrderType,
        max_slippage: i64,
        price: i64,
        margin: i64,
    },
    CancelOrder {
        request_id: String,
        user_id: String,
        order_id: String,
        market: String,
    },
    ClosePosition {
        request_id: String,
        user_id: String,
        market: String,
    },
}

// We also need Error messages

pub enum PublishMessages {
    AddBalance {
        request_id: String,
        msg: String,
        data: UserBalance,
    },
    WithdrawBalance {
        request_id: String,
        msg: String,
        data: UserBalance,
    },
    GetBalance {
        request_id: String,
        msg: String,
        data: UserBalance,
    },
    CreateOrder {},
    CancelOrder {},
    ClosePosition {},
}

#[derive(Serialize, Clone)]
pub struct UserBalance {
    pub available: i64,
    pub locked: i64,
}

pub struct WorkerOutput {
    pub fill: String,
}

pub struct IncomingOrder {
    pub request_id: String,
    pub user_id: String,
    pub qty: i64,
    pub side: Side,
    pub market: String,
    pub order_type: OrderType,
    pub max_slippage: i64,
    pub price: i64,
    pub margin: i64,
}

pub struct MarketData {}