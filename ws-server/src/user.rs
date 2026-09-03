// user.rs

// Represents one WebSocket connection. It should store the connection ID
// and WebSocket sender. It listens for client messages such as SUBSCRIBE and UNSUBSCRIBE,
// then forwards those requests to SubscriptionManager. It should also detect connection
// closure and notify SubscriptionManager so the connection can be removed. It provides a method to send a message to that WebSocket.

// basically here in this file i need to lock in
// on this thing which is what exactly???

// um so i just need to write the user files

use axum::extract::ws::WebSocket;

pub struct User {
    pub id: String,
    pub ws: WebSocket,
}


// impl User {
//     fn new ( id : String , ws : WebSocket)-> User{




//     }


// }




// the thing is we will think of the code organisation later
// right now we will lock in onn thinking from the first principals

// do this part make a user connect to the application
// then save his/her socket id 
// after that just send them a message when a message is coming from the pub sub

// step 1 receive message from the pub sub
// step 2 get a person connect on the web socket
// step 3 just connect that loop 

// after that we will lock in and 
