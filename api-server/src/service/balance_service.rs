#[allow(unused_variables)]
pub async fn add_balance(userid : i32, addamount : i32) -> (StatusCode, OnrampReturnType) {
    let mut users = USERS.lock().unwrap();
    let user = users.iter_mut().find(|u| u.id == userid);

    let redis = get_conn().await;
    
        
    todo!();



}