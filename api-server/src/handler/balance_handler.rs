use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

use crate::{claims::Claims, dto::balance_dto::OnRampRequestPayloadBodyType, service::balance_service::add_balance};

pub async fn on_ramp_handler(
    Extension(claims) : Extension<Claims>,
    Json(body): Json<OnRampRequestPayloadBodyType>
) -> impl IntoResponse {

    match add_balance(claims.user_id , body.amount ) {

        Ok(response) => {
            (StatusCode::OK, Json(response))
        }

        Err(err) => {
            (StatusCode::UNAUTHORIZED, Json(err))
        }
    }
}