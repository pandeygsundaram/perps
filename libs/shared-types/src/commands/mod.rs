pub mod add_balance;
pub mod create_order;
pub mod cancel_order;
pub mod close_position;

use add_balance::AddBalanceCommand;
use create_order::CreateOrderCommand;
use serde::{Deserialize, Serialize};


use crate::commands::{cancel_order::CancelOrderCmd, close_position::ClosePositionCmd};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineCommand {

    AddBalance(AddBalanceCommand),
    CreateOrder(CreateOrderCommand),
    CancelOrder(CancelOrderCmd),
    ClosePosition(ClosePositionCmd)


}