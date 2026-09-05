use futures_util::{StreamExt, stream::SplitSink};

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
    AppState,  types::{ClientMessage, SenderChannelObject, WsMethod}, 
};

pub async fn ws_handler(ws: WebSocketUpgrade, app_state: State<AppState>) -> impl IntoResponse {
    println!("So here we upgraded the socket to the web socket basically");

    ws.on_upgrade(move |socket| handle_socket(socket, app_state))
}

// apparently the axum server spawns multiple tasks for this function
// so all of them live simultaneously waiting for some message to arrive from the user side!

pub async fn handle_socket(mut socket: WebSocket, app_state: State<AppState>) {
    let (mut sender, mut reciever) = socket.split();

    // apparently i have gotten the map
    // but the map needs to be inside the
    // that function

    // now the sender is going to be spawned in a seperate task
    // because we need to do that shit with this human!
    let (tx, rx) = mpsc::unbounded_channel::<SenderChannelObject>();

    let user_id: Uuid = {
        let users = &mut app_state.user_state.write().await;
        users.add_user(tx)
    };

    // this tx will be saved inside the user
    // and the rx will be passed to the receiver

    tokio::spawn(async move { handle_sender(rx, sender, &user_id).await });

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
                        for channel in client_msg.params {
                            println!("SUBSCRIBE: {channel}");

                            // get the write lock of the subscription manager
                            // call the subscribe method






                        }
                        // basically here only we will now start passing things to the
                        // subscription manager
                    }
                    WsMethod::Unsubscribe => {
                        for channel in client_msg.params {
                            println!("UNSUBSCRIBE: {channel}");

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
            
            _ => {}
        }
        
        
    }
    
    {
        let subscriptions =    app_state.subscription_state.write().await ;
        subscriptions.connection_left(  );
    }
    
    
    // get the usermanager access 
    // call the remove user method





    println!("Client disconnected");
}

pub async fn handle_sender(
    rx: UnboundedReceiver<SenderChannelObject>,
    mut sender: SplitSink<WebSocket, Message>,
    user_id: &Uuid,
) {
}
