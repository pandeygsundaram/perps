use std::time::{Duration, Instant};

use futures_util::StreamExt;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("So here we upgraded the socket to the web socket basically");
    ws.on_upgrade(handle_socket)
}

pub async fn handle_socket(mut socket: WebSocket) {
    let mut ping_timer = tokio::time::interval(Duration::from_secs(2));

    let mut last_pong = Instant::now();

    loop {
        tokio::select! {
            msg = socket.next() =>  {
                

                match msg {


                    // This is where the actual frame handling will go.
                    // For learning, try implementing one case at a time:
                    //   1) Text echo
                    //   2) Pong response
                    //   3) Close handling
                    //   4) Binary ignore/handling

                    Some(Ok(Message::Text(text)))=>{
                        let x = format!("message reveived: {text}" );
                        if socket.send(Message::Text(x)).await.is_err() {

                            break;
                        }
                    },

                    Some(Ok (Message::Pong(_)))=>{
                        println!("Pong received!");
                        last_pong = Instant::now();

                        // save the ping time
                        // respond with a pong
                    },

                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) =>{
                        break;
                    },

                    _ => { }
                }
            }
            _ = ping_timer.tick() => {



                // This branch is where heartbeat logic belongs.
                // Typical pattern:
                // - send Ping
                // - check if the last Pong is too old
                // - close the socket if the peer is unresponsive
                //
                // For now, just use this branch to understand that websocket
                // servers are not purely reactive; they often need timers too.



                let _ = last_pong;
            }
        }
    }

    // If we get here, the socket was closed, errored, or the heartbeat failed.

    println!("Client disconnected");
}
