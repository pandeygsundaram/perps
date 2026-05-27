use redis::aio::MultiplexedConnection;

pub async  fn publish_ack(conn : &mut MultiplexedConnection, request_id : &str , status: &str , data: &str ){
    // PUBLISH engine:acks "{request_id}:{status}"

    let msg = format!("{}:{}:{}" , request_id , status , data);
    let _ : () = redis::cmd("PUBLISH")
        .arg("engine:acks")
        .arg(msg)
        .query_async(conn)
        .await
        .unwrap_or(());
} 

pub async fn publish_event(conn : &mut MultiplexedConnection, event_json : &str){
    // XADD engine:events * data "{event_json}"

    let _: () = redis::cmd("XADD")
    .arg("engine:events")
    .arg("*")
    .arg("data")
    .arg(event_json)
    .query_async(conn)
    .await
    .unwrap_or(());

}