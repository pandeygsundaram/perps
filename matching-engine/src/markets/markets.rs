use std::collections::HashMap;

use tokio::sync::mpsc::{self, Sender};

use crate::{
    markets::helper::market_order_sweep,
    types::{OrderType, WorkerCommands, WorkerOutput},
};

pub async fn start_market_workers(
    markets: Vec<String>,
    mut market_senders: HashMap<String, Sender<WorkerCommands>>,
    market_to_fills_processor_sender: Sender<WorkerOutput>,
) -> HashMap<String, Sender<WorkerCommands>> {
    // market threads are all here now
    for i in markets {
        let (mtx, mut mrx) = mpsc::channel::<WorkerCommands>(32);
        market_senders.insert(i, mtx);

        let ftx = market_to_fills_processor_sender.clone();

        tokio::spawn(async move {
            // now here do the logic part
            while let Some(value) = mrx.recv().await {
                // get the btreemap
                // based of the price
                // add the price in there for the open order thingy

                match value {
                    WorkerCommands::CreateOrder {
                        request_id,
                        user_id,
                        qty,
                        side,
                        market,
                        order_type,
                        max_slippage,
                        price,
                        margin,
                    } => {
                        if order_type == OrderType::Limit {
                            // limit order -> put the order in the order book, call try match , get the fills , call the markettofills sender

                            // add the order in the orderbook btich!!
                        } else {
                            // market order
                            let curr_fills = market_order_sweep(
                                book :  ,
                                taker_user_id : user_id,
                                taker_side : side,
                                qty : qty,
                                reference_price ,
                                slippage_bps,
                            ).await;
                        }
                    }
                    WorkerCommands::CancelOrder {
                        request_id,
                        user_id,
                        order_id,
                        order_side,
                        price,
                    } => {}
                    WorkerCommands::GetOrderbook {} => {}
                    WorkerCommands::GetUserOrder {
                        request_id,
                        user_id,
                    } => {}
                    WorkerCommands::UpdateOrder {} => {}
                }

                // first extract the order from the rx
                // create a open order
                // save in orderbook depending upon the order type
                // run try match
                // get the fills
                // call the fills processor

                let x = WorkerOutput {
                    fill: "samosa".to_string(),
                };
                let _ = ftx.send(x).await;
            }
        });
    }

    market_senders
}
