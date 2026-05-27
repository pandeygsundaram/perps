use std::collections::{BTreeMap, HashMap};

use redis;
use tokio::sync::mpsc;

use crate::{
    engine::commands::OrderCommand, models::orderbook::Orderbook,
    workers::market_worker::MarketWorker,
};

mod engine;
mod manager;
mod models;
mod redis_helper;
mod workers;

#[tokio::main]
async fn main() {
    let markets = vec!["BTC", "ETH"];
    // this is a hashmap of all the people who are there to basically
    let mut senders: HashMap<String, mpsc::Sender<OrderCommand>> = HashMap::new();

    for market in markets {
        let (tx, rx) = mpsc::channel(100);

        let worker = MarketWorker {
            market: market.to_string(),
            orderbook: Orderbook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
                last_traded_price: 0,
                index_price: 0,
            },
            receiver: rx,
        };
        tokio::spawn(async move {
            worker.run().await;
        });
        senders.insert(market.to_string(), tx);
    }

    let mut conn = redis_client::get_conn().await;
    let mut last_id = "0".to_string();

    loop {
        let reply: redis::streams::StreamReadReply = redis::cmd("XREAD")
            .arg("COUNT")
            .arg(10)
            .arg("BLOCK")
            .arg(500)
            .arg("STREAMS")
            .arg("engine:commands")
            .arg(&last_id)
            .query_async(&mut conn)
            .await
            .unwrap_or(redis::streams::StreamReadReply { keys: vec![] });

        for stream_key in &reply.keys {
            for message in &stream_key.ids {
                last_id = message.id.clone();

                if let Some(data) = message.map.get("data") {
                    let json: String = redis::from_redis_value(data).unwrap_or_default();
                    let cmd: Result<shared_types::commands::EngineCommand, serde_json::Error> =
                        serde_json::from_str(&json);
                    match cmd {
                        Ok(shared_types::commands::EngineCommand::AddBalance(payload)) => {
                            let user_id = payload.user_id;
                            manager::balance_manager::process_add_balance(payload, &mut conn).await;
                            redis_helper::publisher::publish_ack(
                                &mut conn,
                                "na",
                                "ok",
                                &format!("balance_added:{}", user_id),
                            )
                            .await;
                        }
                        Ok(shared_types::commands::EngineCommand::CreateOrder(payload)) => {
                            let market = payload.market.clone();
                            let user_id = payload.user_id;
                            let price = payload.price;
                            let qty = payload.qty;
                            let margin = payload.margin;
                            let side = payload.side.clone();

                            match manager::order_manager::process_add_order(payload) {
                                Ok(()) => {
                                    if let Some(sender) = senders.get(&market) {
                                        let _ = sender
                                            .send(OrderCommand::CreateOrder {
                                                order_id: 0,
                                                user_id,
                                                side,
                                                price,
                                                qty,
                                                margin,
                                            })
                                            .await;
                                        redis_helper::publisher::publish_ack(
                                            &mut conn,
                                            "na",
                                            "ok",
                                            &format!("order_placed:{}", user_id),
                                        )
                                        .await;
                                        redis_helper::publisher::publish_event(&mut conn, &format!(r#"{{"type":"OrderPlaced","user_id":{},"market":"{}","price":{},"qty":{}}}"#, user_id, market,  price, qty)).await;
                                    }
                                }
                                Err(manager::order_manager::OrderError::InsufficientBalance) => {
                                    println!("Insufficient Balance")
                                }
                                Err(manager::order_manager::OrderError::UserNotFound) => {
                                    println!("UserNotFound")
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }
}
