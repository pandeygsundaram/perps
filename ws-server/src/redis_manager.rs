use futures_util::StreamExt;



pub async fn start_redis() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut pubsub = client.get_async_pubsub().await?;

    pubsub.subscribe("channel_1").await?;
    pubsub.subscribe("channel_2").await?;

    loop {
        let msg = pubsub.on_message().next().await.unwrap();

        let payload: String = msg.get_payload().unwrap();

        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}
