use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosePositionCmd {
       request_id : String,
     user_id : i64,
    market: String,
    //   slippage_bps: i64, // slippage tolerance for the opposing market order
}