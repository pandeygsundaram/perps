use redis::{Client, aio::MultiplexedConnection};


pub async fn get_conn() -> MultiplexedConnection {
    let client = Client::open("redis://127.0.0.1/").unwrap();

    client
        .get_multiplexed_async_connection()
        .await
        .unwrap()

} 


