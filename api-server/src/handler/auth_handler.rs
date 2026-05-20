use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::dto::auth_dto::{ LoginRequestPayloadBodyType, SignupRequestPayloadBodyType, };
use crate::service::auth_service::{login, signup};

pub async fn login_handler(
    Json(cred): Json<LoginRequestPayloadBodyType>
) -> impl IntoResponse {

    match login(cred) {

        Ok(response) => {
            (StatusCode::OK, Json(response))
        }

        Err(err) => {
            (StatusCode::UNAUTHORIZED, Json(err))
        }
    }
}

pub async fn signup_handler(
    Json(cred) :Json<SignupRequestPayloadBodyType>
) -> impl IntoResponse {

    match signup(cred) {

        Ok(response) => {
            (StatusCode::OK, Json(response))
        }

        Err(err) => {
            (StatusCode::UNAUTHORIZED, Json(err))
        }
    }
}