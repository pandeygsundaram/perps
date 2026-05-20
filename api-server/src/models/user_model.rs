use serde::{Deserialize, Serialize};

#[derive(Clone , Serialize , Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub balance : Balance,
    #[serde(skip_serializing)]
    pub password: String,
}


#[derive(Clone , Serialize , Deserialize)]
pub struct Balance {
    pub available: i32,
    pub locked : i32
}