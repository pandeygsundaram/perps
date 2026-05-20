use crate::{ dto::auth::{LoginCredential, LoginReturnType, UserDetails}, models::user_model::User};

#[allow(unused_variables)]
pub fn login(
    cred: LoginCredential
) -> Result<LoginReturnType, LoginReturnType> {
    todo!()

    // let users = USERS.lock().unwrap();

    // let user = users
    //     .iter()
    //     .find(|u| u.username == cred.username);

    // match user {

    //     Some(u) if u.password == cred.password => {

    //         let token = create_token(u.id);

    //         Ok(LoginReturnType {
    //             message: "Loggedin Successfully".to_string(),
    //             token: Some(token),
    //             data: Some(u.clone()),
    //         })
    //     }

    //     _ => {
    //         Err(LoginReturnType {
    //             message: "Invalid Credentials".to_string(),
    //             token: None,
    //             data: None,
    //         })
    //     }
    // }
}




#[allow(unused_variables)]
pub fn signup(user_details: UserDetails) -> User {
    todo!();
    // let mut users = USERS.lock().unwrap();

    // let last_id = users.last().map(|u| u.id).unwrap_or(0);

    // let new_user = User {
    //     id: last_id + 1,
    //     name: user_details.name,
    //     password: user_details.password,
    //     balance: Balance { available: 0, locked: 0 },
    //     username: user_details.username,
    // };

    // users.push(new_user.clone());
    // new_user
}




