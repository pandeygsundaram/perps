use serde::{Deserialize, Serialize};
use crate::models::user_model::User;

#[derive(Deserialize)]
pub struct  SignupRequestPayloadBodyType{
    username:String,
    name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequestPayloadBodyType {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginRequestReturnType {
    message: String,
    data : Option<User> ,
    token : Option<String>
}