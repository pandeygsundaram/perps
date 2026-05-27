use crate::{ dto::auth::{LoginCredential, LoginReturnType, UserDetails}, models::user_model::User};

#[allow(unused_variables)]
pub fn login(
    cred: LoginCredential
) -> Result<LoginReturnType, LoginReturnType> {
    todo!()

  
}




#[allow(unused_variables)]
pub fn signup(user_details: UserDetails) -> User {
    todo!();
    
}




