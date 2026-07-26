use std::{collections::HashMap, sync::mpsc::Sender, time::Duration};

use redis::Value;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::types::{
    DispatcherToSettlementChannelProp, EngineCommands, ErrorOrigin, IncomingOrder, PublishMessages,
    UserBalance, WorkerCommands,
};

pub async fn start_settlement(
    mut user_balances_store: HashMap<String, UserBalance>,
    market_senders: HashMap<String, Sender<WorkerCommands>>,
    tx2: Sender<PublishMessages>,
    mut rx: Receiver<DispatcherToSettlementChannelProp>,
    settlement_to_dispatcher_sender: Sender<HashMap<String, Value>>
) {
    while let Some(event) = rx.recv().await {
        let Some(data) = event.data.get("data") else {
            send_error_msg_and_ack(
                "DATA FIELD NOT FOUND IN THE REDIS COMMAND".to_string(),
                event,
                &tx2,
                ErrorOrigin::Settlement,
            )
            .await;
            continue;
        };

        // did something wierdly converting Type Value to string
        let data_str = match data {
            redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone()).unwrap(),
            _ => {
                send_error_msg_and_ack(
                    "UnExpected value type received".to_string(),
                    event,
                    &tx2,
                    ErrorOrigin::Settlement,
                )
                .await;
                continue;
            }
        };

        let Ok(cmd) = serde_json::from_str::<EngineCommands>(&data_str) else {
            send_error_msg_and_ack(
                "Failed to Parse the variables".to_string(),
                event,
                &tx2,
                ErrorOrigin::Settlement,
            )
            .await;
            continue;
        };

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
                // already data is parsed , but we'll have to check it if these values exist or not! and
                // throw error if they don't

                let Some(user_balance) = user_balances_store.get_mut(&user_id) else {
                    // throw errow user not found!
                    send_error_msg_and_ack(
                        "User not found".to_string(),
                        event,
                        &tx2,
                        ErrorOrigin::Settlement,
                    )
                    .await;
                    continue;
                };
                // check if user has money or not
                if user_balance.available < margin {
                    send_error_msg_and_ack(
                        "Not Enough Funds to place this order".to_string(),
                        event,
                        &tx2,
                        ErrorOrigin::Settlement,
                    )
                    .await;
                    continue;
                }

                // deduct the margin for the user
                user_balance.available -= margin;
                user_balance.locked += margin;

                println!("USER MONEY DEDUCTED SUCCESSFULLY");

                // create open order object!
                // send it to the marker worker

                // exctract market sender
                let Some(curr_sender) = market_senders.get(&market) else {
                    send_error_msg_and_ack(
                        "Market Does not Exists".to_string(),
                        event,
                        &tx2,
                        ErrorOrigin::Settlement,
                    )
                    .await;
                    continue;
                };

                curr_sender
                    .send(WorkerCommands::CreateOrder {
                        request_id,
                        user_id,
                        qty,
                        side,
                        market,
                        order_type,
                        max_slippage,
                        price,
                        margin,
                    })
                    .await;

                // get rid of one shot channel and let's make it a mpsc!

                // if the order is market -> wait for reply from the fills processor
                // for the amount of fills processed and the not filled quantity is going to be returned !!
                // update the user balance and send the ack to the dispatcher

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

pub async fn send_error_msg_and_ack(
    msg: String,
    event: DispatcherToSettlementChannelProp,
    message_publisher: &Sender<PublishMessages>,
    origin: ErrorOrigin,
) {
    // send a message to the publisher for that request id
    message_publisher
        .send(PublishMessages::ErrorEncountered { msg, origin })
        .await;

    // send a ack via the one shot !
    event.reply.send(event.data);
}
