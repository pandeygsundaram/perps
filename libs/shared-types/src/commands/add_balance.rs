use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddBalanceCommand {

    pub user_id: i64,

    pub amount: i64,
}