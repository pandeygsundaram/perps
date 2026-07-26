use std::collections::HashMap;

use crate::dispatcher::process_pending_and_current;
use crate::fills_processor::start_fills_processing;
use crate::markets::markets::start_market_workers;
use crate::publisher::publish_back_to_engine;
use crate::settlement::{self, start_settlement};
use crate::types::{
    DispatcherToSettlementChannelProp, FillEvent, Orderbook, PublishMessages, Status, UserBalance,
    WorkerOutput,
};
use redis::Value;
use tokio::sync::mpsc;

pub async fn start_processing() {
    // from dispatcher thread to settlement thread
    let (tx, rx) = mpsc::channel::<DispatcherToSettlementChannelProp>(32);
    let (settlement_to_dispatcher_sender, settlement_to_dispatcher_receiver) = mpsc::channel::<HashMap<String, Value>>(32);

    // from settlement,fills processor thread to publisher thread
    let (tx2, rx2) = mpsc::channel::<PublishMessages>(32);

    let (market_to_fills_processor_sender, _) = mpsc::channel::<WorkerOutput>(32);

    // dispatcher thread
    tokio::spawn(async move {
        process_pending_and_current(tx , settlement_to_dispatcher_receiver ).await;
    });

    // also make a third thread settlement thread!
    // so for for part settlement thread will give
    // completed flag to the dispatcher

    let user_balances_store: HashMap<String, UserBalance> = HashMap::new();
    // if exists_backup {
    //     Balances = Last_Backedup_Balances;
    // }
    let mut market_senders = HashMap::new();

    let markets = Vec::from(["BTC".to_string(), "ETH".to_string(), "SOL".to_string()]);

    // start the workers

    market_senders =
        start_market_workers(markets, market_senders, market_to_fills_processor_sender).await;

    //Settlement thread
    tokio::spawn(async move {
        // it will get the info from dispatcher and has to do some operation and respond back

        // types of commands we are getting here
        // add balance
        // withdraw balance
        // create order
        // close order
        // close position

        start_settlement(user_balances_store, market_senders, tx2, rx , settlement_to_dispatcher_sender).await;
    });

    //

    // fills processor thread
    tokio::spawn(async move {
        start_fills_processing().await;
    });

    // pub sub and stream publisher task/thread
    tokio::spawn(async move {
        publish_back_to_engine(rx2).await;
    });
}
