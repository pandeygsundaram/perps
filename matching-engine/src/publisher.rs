use redis::AsyncCommands;
use tokio::sync::mpsc::Receiver;

use crate::types::PublishMessages;

pub async fn publish_back_to_engine(mut rx2: Receiver<PublishMessages>) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    while let Some(data) = rx2.recv().await {
        match data {
            PublishMessages::AddBalance {
                request_id,
                msg,
                data,
            } => {
                let response = serde_json::json!(
                    {
                        "request_id":request_id,
                        "msg":msg ,
                        "data": data
                    }
                );
                con.publish::<&str, std::string::String, String>("mychannel", response.to_string())
                    .await
                    .unwrap();

                // let result = con
                //     .xack::<&[&str; 1], &[&str; 1], std::string::String, String>(
                //         &["mystream"],
                //         &["firstconsumer"],
                //         id,
                //     )
                //     .await;
            }
            PublishMessages::GetBalance {
                request_id,
                msg,
                data,
            } => {
                let response = serde_json::json!(
                    {
                        "request_id":request_id,
                        "msg":msg ,
                        "data": data
                    }
                );
                con.publish::<&str, std::string::String, String>("mychannel", response.to_string())
                    .await
                    .unwrap();

                // let result = con
                //     .xack::<&[&str; 1], &[&str; 1], std::string::String, String>(
                //         &["mystream"],
                //         &["firstconsumer"],
                //         id,
                //     )
                //     .await;
            }
            PublishMessages::WithdrawBalance {
                request_id,
                msg,
                data,
            } => {
                let response = serde_json::json!(
                    {
                        "request_id":request_id,
                        "msg":msg ,
                        "data": data
                    }
                );
                con.publish::<&str, std::string::String, String>("mychannel", response.to_string())
                    .await
                    .unwrap();

                // let result = con
                //     .xack::<&[&str; 1], &[&str; 1], std::string::String, String>(
                //         &["mystream"],
                //         &["firstconsumer"],
                //         id,
                //     )
                //     .await;
            }
            _ => {}
        };
    }
}
