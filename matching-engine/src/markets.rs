use std::collections::HashMap;

use tokio::sync::mpsc::{self, Sender};

use crate::types::{IncomingOrder, OrderType, WorkerOutput};

pub async fn start_market_workers(
    markets: Vec<String>,
    mut market_senders: HashMap<String, Sender<IncomingOrder>>,
    market_to_fills_processor_sender: Sender<WorkerOutput>,
) -> HashMap<String ,Sender<IncomingOrder> > {
    // market threads are all here now
    for i in markets {
        let (mtx, mut mrx) = mpsc::channel::<IncomingOrder>(32);
        market_senders.insert(i, mtx);

        let ftx = market_to_fills_processor_sender.clone();

        tokio::spawn(async move {
            // now here do the logic part
            while let Some(value) = mrx.recv().await {
                // get the btreemap
                // based of the price
                // add the price in there for the open order thingy

                if value.order_type == OrderType::Market {
                    // let res =  market_order_sweep(book, taker_user_id, taker_side, qty, reference_price, slippage_bps);

                    // send the data to the processor queue
                } else {

                    // add the new order in the orderbook
                    // place a simple nice and sweet limit order
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
