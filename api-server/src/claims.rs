use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};


#[derive(Debug , Serialize,Clone, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp : usize,
}

pub fn create_token(user_id: i32) -> String{

    let expiration = Utc::now() + Duration::hours(24);
    
    let claims = Claims {
        user_id,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            "secret".as_ref()
        ),
    ).unwrap()


}


pub async fn auth_middleware(
    mut req:Request,
    next:Next,
)-> Result<Response, StatusCode>{
    let auth_headers = req.headers().get("Authorization");

    match auth_headers {
        Some(header) =>{
            let token = header.to_str().unwrap().replace("Bearer ","");
            println!("{}" , token);

            match verify_token(token){
                Some(claims)=>{
                    // attach user_id to req so handlers can read it
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await) 
                }
                None => Err(StatusCode::UNAUTHORIZED)
            }

        }
        None => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

}


pub fn verify_token(token: String) -> Option<Claims>{
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()

    ); 
    match result {
        Ok(token_data)=>Some(token_data.claims),
        Err(_)=> None,
    }

}