pub mod add_balance;
pub mod create_order;

use add_balance::AddBalanceCommand;
use create_order::CreateOrderCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EngineCommand {

    AddBalance(AddBalanceCommand),

    CreateOrder(CreateOrderCommand),
}