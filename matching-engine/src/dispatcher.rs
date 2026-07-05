use std::collections::HashMap;

use redis::{
    AsyncCommands, Value,
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
};
use tokio::sync::oneshot;

use crate::types::DispatcherToSettlementChannelProp;

pub async fn process_pending_and_current(
    // con: &mut MultiplexedConnection,
    tx: tokio::sync::mpsc::Sender<DispatcherToSettlementChannelProp>,
) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    read_and_process_redis(&mut con, true, tx.clone()).await;
    read_and_process_redis(&mut con, false, tx).await;
}

async fn read_and_process_redis(
    con: &mut MultiplexedConnection,
    process_pending_first: bool,
    tx: tokio::sync::mpsc::Sender<DispatcherToSettlementChannelProp>,
) {
    loop {
        let start_from = if process_pending_first { "0" } else { ">" };
        if start_from == "0" {
            println!("PROCESSING THE PENDING QUEUE");
        } else {
            println!("PROCESSING THE CURRENT QUEUE");
        }

        let result: Result<StreamReadReply, redis::RedisError> = con
            .xread_options(
                &["mystream"],
                &[start_from],
                &StreamReadOptions::default()
                    .group(&["firstconsumer"], &["worker1"])
                    .count(10),
            )
            .await;
        let Ok(x) = result else {
            let Err(e) = result else { unreachable!() };
            panic!("{:?}", e);
        };

        println!("{:?}", x);

        // let's send the ack then
        if x.keys.is_empty() || x.keys[0].ids.len() == 0 {
            println!("Pending messages cleared");
            break;
        }

        // extract the id of the streamReply
        // xadd the answer in the queue & publish in the pubsub

        // just simply extract all the values
        // send all sorts of things you'll be sending from the api server
        // log it properly here

        // then publish it
        for i in x.keys {
            println!("{:?}", i.ids[0].id);

            let curr_map = i.ids[0].map.clone();

            fire_and_wait_for_responce_from_settlement(tx.clone(), curr_map).await;

            // redis gives value type so yuu have to convert it to normal string

            let id = &[i.ids[0].id.clone()];

            // println!("{:?}" , i.ids[0].map)
            // just publish the ack for it

            // first pub json to that

            let response = serde_json::json!({
                "status": "ok",
                "request_id":"1"
            });

            send_ack_and_publish_pubsub(con, response, id).await;
        }
    }
}

async fn fire_and_wait_for_responce_from_settlement(
    tx: tokio::sync::mpsc::Sender<DispatcherToSettlementChannelProp>,
    curr_map: HashMap<String, Value>,
) {
    let (otx, orx) = oneshot::channel::<HashMap<String, Value>>();

    let _ = tx
        .send(DispatcherToSettlementChannelProp {
            data: curr_map,
            reply: otx,
        })
        .await;

    let result = orx.await.unwrap();
    println!("{:?}", result);
    println!("RESPONSE CAME SUCCESSFULLY FROM THE SETTLEMENT THREAD")
    // do something with the value now
}

async fn send_ack_and_publish_pubsub(
    con: &mut MultiplexedConnection,
    response: serde_json::Value,
    id: &[String; 1],
) {
    con.publish::<&str, std::string::String, String>("mychannel", response.to_string())
        .await
        .unwrap();

    let result = con
        .xack::<&[&str; 1], &[&str; 1], std::string::String, String>(
            &["mystream"],
            &["firstconsumer"],
            id,
        )
        .await;

    println!("{:?}", result);
    println!("Ack sent successful");
}
