use futures_util::{SinkExt, StreamExt, stream::SplitSink};

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use uuid::Uuid;

use crate::{
    AppState, types::{ClientMessage,  WsMethod}, user_manager::UserManager,
};

pub async fn ws_handler(ws: WebSocketUpgrade, app_state: State<AppState>) -> impl IntoResponse {
    println!("So here we upgraded the socket to the web socket basically");

    ws.on_upgrade(move |socket| handle_socket(socket, app_state))
}

// apparently the axum server spawns multiple tasks for this function
// so all of them live simultaneously waiting for some message to arrive from the user side!

pub async fn handle_socket(socket: WebSocket, app_state: State<AppState>) {
    let (sender, mut reciever) = socket.split();

    // apparently i have gotten the map
    // but the map needs to be inside the
    // that function

    // now the sender is going to be spawned in a seperate task
    // because we need to do that shit with this human!
    let (tx, rx) = mpsc::unbounded_channel::<String>();

    let user_id: Uuid = {
        let users = &mut app_state.user_state.write().await;
        users.add_user(tx)
    };

    // this tx will be saved inside the user
    // and the rx will be passed to the receiver

    tokio::spawn(async move { handle_sender(rx, sender).await });

    // call the save user method for this particular tx

    // basically the thing is that i have to next go ahead and lock in
    // so now i will go ahead and write those methods/funcitons

    loop {
        let msg = reciever.next().await;

        match msg {
            Some(Ok(Message::Text(text))) => {
                let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) else {
                    panic!("Wrong evenet found");
                };

                match client_msg.method {
                    WsMethod::Subscribe => {
                        let mut subs_handler = app_state.subscription_state.write().await;
                        for channel in client_msg.params {
                            println!("SUBSCRIBE: {channel}");


                            // get the subscription manager lock
                            // subscribe the user

                            subs_handler.subscribe(user_id, channel);



                            // get the write lock of the subscription manager
                            // call the subscribe method
                        }
                        // basically here only we will now start passing things to the
                        // subscription manager
                    }
                    WsMethod::Unsubscribe => {
                        for channel in client_msg.params {
                            println!("UNSUBSCRIBE: {channel}");


                            let mut subs_handler = app_state.subscription_state.write().await;
                            subs_handler.unsubscribe(user_id , channel);


                            // get the write lock
                            // call the unsubscribe method
                        }

                        // again here also it will pass that data to the
                        // unsubscribe thingy
                    }
                }

                // if socket.send(Message::Text(xy)).await.is_err() {
                //     break;
                // }
            }

            Some(Ok(Message::Close(_))) | None | Some(Err(_)) => {
                break;
            }

            _ => {
                println!("Something wierd happening")
            }
        }
    }

    {
        // unsubscribe + delete via usermanager
        let subscriptions = &mut app_state.subscription_state.write().await;
        subscriptions.connection_left(user_id).await;
    }
    // remove the user from the user map
    {
        let user_handler = &mut UserManager::get_instance().write().await;
        user_handler.remove_user(user_id);
    }

    println!("Client disconnected");
}

pub async fn handle_sender(
    mut rx: UnboundedReceiver<String>,
    mut sender: SplitSink<WebSocket, Message>,
) {
    while let Some(event) = rx.recv().await {
        if let Err(e) = sender.send(Message::Text(event.into())).await {
            eprintln!("Failed to send WebSocket message: {e}");
            break;
        }
    }

    // just need to wrap up this one now
    // we don't need the user_id
    // cause the sender itself belongs to that user socket
}
