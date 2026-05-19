use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{USERS, claims::create_token};

#[derive(Clone , Serialize , Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub balance : i32,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize)]
pub struct  UserDetails{
    username:String,
    name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginCredential {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginReturnType {
    message: String,
    data : Option<User> ,
    token : Option<String>
}



fn login( cred: LoginCredential) -> Option<User> {
    let users  = USERS.lock().unwrap();
    let user = users.iter().find(|u| u.username == cred.username);

    match user {
        Some(u)=> {
            print!("User found, let's match password");
            if u.password == cred.password {
                Some(u.clone())
            }else {
                return None;
            }
        },
        _ =>{
            print!("User not found bitch");
            return None;
        }
    }

}

fn signup(user_details: UserDetails) -> User {
    let mut users = USERS.lock().unwrap();

    let last_id = users.last().map(|u| u.id).unwrap_or(0);

    let new_user = User {
        id: last_id + 1,
        name: user_details.name,
        password: user_details.password,
        balance: 0,
        username: user_details.username,
    };

    users.push(new_user.clone());
    new_user
}


pub async fn signup_handler(Json(user_details): Json<UserDetails>) -> (StatusCode, Json<LoginReturnType>) {
    let user = signup(user_details);
    let token = create_token(user.id);

    let response = LoginReturnType {
        message: "User created successfully".to_string(),
        token: Some(token),
        data: Some(user),
    };

    (StatusCode::CREATED, Json(response))
}


pub async fn login_handler( Json(cred) : Json<LoginCredential>) ->(StatusCode, Json<LoginReturnType>) {

    match  login(cred) {
        Some(u) =>{
            println!("{}",u.name);
            let token = create_token(u.id);
            let responce = LoginReturnType {
                message: "Loggedin Successfully".to_string(),
                token: Some(token),
                data: Some(u)
            };
            return (StatusCode::OK , Json(responce)); 
        },
        _ => {
            let responce = LoginReturnType{
                message: "Invalid Credentials".to_string(),
                data : None,
                token: None,
            };
            return (StatusCode::UNAUTHORIZED, Json(responce));
        }   
    }
}
