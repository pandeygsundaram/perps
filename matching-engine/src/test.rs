use futures_util::StreamExt;
use redis::AsyncCommands;

use crate::start_processing;

#[tokio::test]
pub async fn  test_vloop() {

    // in the
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let Ok(mut con) = client.get_multiplexed_async_connection().await else {
        panic!("Redis is not live")
    };
    let Ok(mut pubsubcon) = client.get_async_pubsub().await else {
        panic!("Redis pubsub acquire failed");
    };
    const STREAM: &str = "mystream";


    // subscribe simply to the channel
    let _ = pubsubcon.subscribe("mychannel").await;

    //now accumulate the messages
    let mut messages = pubsubcon.on_message();

    // con.xadd( &[STREAM] , id, items).await;

    let cmd1 = serde_json::json!({
        "command":"add_balance",
        "user_id":"iunfndi",
        "request_id":"sfdsf",
        "amount": 400
    })
    .to_string();

    let result = con
        .xadd::<&[&str; 1], &str, &str, String, String>(&[STREAM], "*", &[("data", cmd1)])
        .await;
    //   CreateOrder   { request_id, user_id, market, side, order_type, price, qty, margin }

    // let result2 = con
    //     .xadd::<&[&str; 1], &str, &str, String, String>(
    //         &[STREAM],
    //         "*",
    //         &[
    //             ("command", "create_order".to_string()),
    //             ("request_id", 2.to_string()),
    //             ("user_id", 1.to_string()),
    //             ("side", "Long".to_string()),
    //             ("market", "BTC".to_string()),
    //             ("order_type", "Limit".to_string()),
    //             ("price", 100.to_string()),
    //             ("qty", 10.to_string()),
    //             ("margin", 10.to_string()),
    //         ],
    //     )
    //     .await;

    // // this got published then godda wait for it in pubsub basically

    // let result3 = con
    //     .xadd::<&[&str; 1], &str, &str, String, String>(
    //         &[STREAM],
    //         "*",
    //         &[
    //             ("command", "create_order".to_string()),
    //             ("request_id", 2.to_string()),
    //             ("user_id", 1.to_string()),
    //             ("market", "ETH".to_string()),
    //             ("side", "Long".to_string()),
    //             ("order_type", "Limit".to_string()),
    //             ("price", 100.to_string()),
    //             ("qty", 10.to_string()),
    //             ("margin", 10.to_string()),
    //         ],
    //     )
    //     .await;

    // let result4 = con
    //     .xadd::<&[&str; 1], &str, &str, String, String>(
    //         &[STREAM],
    //         "*",
    //         &[
    //             ("command", "cancel_order".to_string()),
    //             ("user_id", 1.to_string()),
    //             ("order_id", "1".to_string()),
    //             ("request_id", 3.to_string()),
    //         ],
    //     )
    //     .await;

    let cmd2 = serde_json::json!({
        "command":"add_balance",
        "user_id":"ifjsgnijs",
        "request_id":"sfdsf",
        "amount": 400
    })
    .to_string();

    let result5 = con
        .xadd::<&[&str; 1], &str, &str, String, String>(&[STREAM], "*", &[("data", cmd2)])
        .await;

    // let result6 = con
    //     .xadd::<&[&str; 1], &str, &str, String, String>(
    //         &[STREAM],
    //         "*",
    //         &[
    //             ("command", "close_position".to_string()),
    //             ("user_id", 1.to_string()),
    //             ("request_id", 5.to_string()),
    //         ],
    //     )
    //     .await;

    let Ok(x) = result else {
        let Err(e) = result else { unreachable!() };
        panic!("{:?}", e);
    };

    println!("Item Saved in redis stream for add balance");
    println!("{}", x);

    let joinhandler = tokio::spawn(async move {
        start_processing().await;
        println!("LOOP EXITED SUCCESSFULLY")
    });

    // now listen to there message ids and basically tell if they got done or not?
    println!("LISTENING TO THE INCOMING MESSAGES");

    while let Some(msg) = messages.next().await {
        let payload: String = msg.get_payload().unwrap();

        println!("Payload from pubsubreceived {}", payload);
    }

    joinhandler.await.unwrap()
}
