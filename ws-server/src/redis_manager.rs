use futures_util::StreamExt;
use tokio::sync::RwLock;

use crate::{subscription_manager::SubscriptionManager, types::MarketEvents};

pub async fn start_redis(
    _subs_state: &RwLock<SubscriptionManager>,
    markets: Vec<String>,
) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut pubsub = client.get_async_pubsub().await?;

    // there are going to be bunch of markets in shared types
    // pick up markets from there
    // subscribe to their these types of events

    let channels = [
        "trades".to_string(),
        "depth".to_string(),
        "ticker".to_string(),
        "derivativeticker".to_string(),
    ];
    // and pass it to the subscription manager

    let final_channel: Vec<String> = channels
        .iter()
        .flat_map(|x| markets.iter().map(move |y| format!("{}.{}", x, y)))
        .collect();

    for channel in &final_channel {
        pubsub.subscribe(channel).await?;
    }

    loop {
        let msg = pubsub.on_message().next().await.unwrap();

        let payload: String = msg.get_payload().unwrap();

        println!("channel '{}': {}", msg.get_channel_name(), payload);

        // channel is basically like 
        // eventtype . market

        // so we need to filter it like that






        let Ok(events) = serde_json::from_str::<MarketEvents>(&payload) else {
            panic!("Wrong evenet found")
        };
        println!("{events :?}");

        // so we have found the events we just have to pass
        // the events to these to the subscription manager simply
        // and that manager is going to pass it to all the
        // people connected on the server

        // all the channels are going to be all the markets
        // in our  current exchange
        // that thing should basically come in shared lib folder

        // pass events to user_manager thread
    }
}
