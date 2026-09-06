use futures_util::StreamExt;

use crate::{
    subscription_manager::SubscriptionManager, types::MarketEvents, user_manager::UserManager,
};

pub async fn start_redis(markets: Vec<String>) -> redis::RedisResult<()> {
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

        // get the instance of the subscription manager
        // and call the broadcast method!


        // will have to parse the data
        // like the senderchannelobject needs to be modified!
        // and we also need to do some kinda processing on the event types and the actual senderchannelobject

        let Ok(event) = serde_json::from_str::<MarketEvents>(&payload) else {
            eprintln!("Wrong evenet found");
            continue;
        };

        let Ok(message) = serde_json::to_string(&event) else {
            eprintln!("Failed to serialize market event");
            continue;
        };

        // took lock of the subs manager and then got rid of it

        let users = {
            let subs = SubscriptionManager::get_instance().read().await;
            subs.broadcast(msg.get_channel_name().to_string()).ok()
        };

        // now here tool lock of user manager and got rid of it once events are sent
        if let Some(users) = users {
            let user_handler = UserManager::get_instance().read().await;
            for id in users {
                user_handler.emit(&id, message.clone());
            }
        }

        // now here we will have to build out own sender channel object

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
