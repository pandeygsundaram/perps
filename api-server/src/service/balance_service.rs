use axum::http::StatusCode;

use crate::USERS;

#[allow(unused_variables)]
pub async fn add_balance(userid : i32, addamount : i32) -> (StatusCode, OnrampReturnType) {
    let mut users = USERS.lock().unwrap();
    let user = users.iter_mut().find(|u| u.id == userid);

    let redis = get_conn().await;

    // so yeah here simply add the item in the redis and wait for the responce
    let res = api_loopback( AddBalance {userid , addamount} )
    
    
        
    todo!();



}

// it will take the req body and command type
pub async fn api_loopback(){







} 