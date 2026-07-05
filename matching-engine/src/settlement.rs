use std::{collections::HashMap, time::Duration};

use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::types::{
    DispatcherToSettlementChannelProp, EngineCommands, IncomingOrder, PublishMessages, UserBalance,
};

pub async fn start_settlement(
    mut user_balances_store: HashMap<String, UserBalance>,
    market_senders : HashMap<String , Sender<IncomingOrder>>,
    tx2: Sender<PublishMessages>,
    mut rx: Receiver<DispatcherToSettlementChannelProp>,
) {
    while let Some(event) = rx.recv().await {
        let Some(data) = event.data.get("data") else {
            panic!("DATA FIELD NOT FOUND IN THE REDIS COMMAND")
        };

        // did something wierdly converting Type Value to string
        let data_str = match data {
            redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone()).unwrap(),
            _ => panic!("unexpected value type"),
        };

        let cmd = serde_json::from_str::<EngineCommands>(&data_str).unwrap();

        match cmd {
            EngineCommands::AddBalance {
                request_id,
                user_id,
                amount,
            } => {
                println!("Add balance command received");

                let user_balance = if user_balances_store.get_mut(&user_id).is_some() {
                    user_balances_store.get_mut(&user_id).unwrap()
                } else {
                    user_balances_store.insert(
                        user_id.clone(),
                        UserBalance {
                            available: 0,
                            locked: 0,
                        },
                    );
                    user_balances_store.get_mut(&user_id).unwrap()
                };
                user_balance.available += amount;

                // send the message to published task
                let _ = tx2
                    .send(PublishMessages::AddBalance {
                        request_id,
                        msg: "Balance Updated successfully".to_string(),
                        data: user_balance.clone(),
                    })
                    .await;

                // send the events to the publish queue
                // send back the ack

                event.reply.send(event.data).unwrap();
            }
            EngineCommands::WithdrawBalance {
                request_id,
                user_id,
                amount,
            } => {
                let user_balance = if user_balances_store.get_mut(&user_id).is_some() {
                    user_balances_store.get_mut(&user_id).unwrap()
                } else {
                    user_balances_store.insert(
                        user_id.clone(),
                        UserBalance {
                            available: 0,
                            locked: 0,
                        },
                    );
                    user_balances_store.get_mut(&user_id).unwrap()
                };
                if user_balance.available < amount {
                    // publish the error

                    let _ = tx2
                        .send(PublishMessages::WithdrawBalance {
                            request_id,
                            msg: "Not Sufficient Balance".to_string(),
                            data: user_balance.clone(),
                        })
                        .await;
                    event.reply.send(event.data).unwrap();
                } else {
                    user_balance.available -= amount;

                    // send the message to published task
                    let _ = tx2
                        .send(PublishMessages::WithdrawBalance {
                            request_id,
                            msg: "Balance Updated successfully".to_string(),
                            data: user_balance.clone(),
                        })
                        .await;

                    // send the events to the publish queue
                    // send back the ack

                    event.reply.send(event.data).unwrap();
                }
            }
            EngineCommands::GetBalance {
                request_id,
                user_id,
            } => {
                let user_balance = if user_balances_store.get_mut(&user_id).is_some() {
                    user_balances_store.get_mut(&user_id).unwrap()
                } else {
                    user_balances_store.insert(
                        user_id.clone(),
                        UserBalance {
                            available: 0,
                            locked: 0,
                        },
                    );
                    user_balances_store.get_mut(&user_id).unwrap()
                };

                let _ = tx2
                    .send(PublishMessages::GetBalance {
                        request_id,
                        msg: "Balance fetched successfully".to_string(),
                        data: user_balance.clone(),
                    })
                    .await;

                event.reply.send(event.data).unwrap();
            }

            EngineCommands::CreateOrder {
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

                // depending upon the market just simply send the order !!
                // here get the correct tx for the market
                // then basically push it in there

                // then the market is going to basically pick up the shit
                // and then it is going to try_match it
                // return the fills to the fills processor

                println!("USER MONEY DEDUCTED SUCCESSFULLY");
            }

            EngineCommands::CancelOrder {
                request_id,
                user_id,
                order_id,
                market,
            } => {
                
                println!("Create order command received");
                // println!("{:?}", c);
                // fire_and_wait_for_responce_from_settlement(tx.clone(), curr_map).await;
            }

            EngineCommands::ClosePosition {
                request_id,
                user_id,
                market,
            } => {
                println!("Cancel order command received");
                // println!("{:?}", c);
                // fire_and_wait_for_responce_from_settlement(tx.clone(), curr_map).await;
            } // _ => {
              //     println!("Stupid ass commands reject it");
              // } // println!("{:?}", i.0);
        }

        // let _ = event.reply.send(data);

        println!("REPLY SENT FROM THE SETTLEMENT THREAD");
        sleep(Duration::from_secs(2)).await;
    }
}
