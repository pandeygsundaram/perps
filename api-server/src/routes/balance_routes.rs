use axum::{routing::post,Router,};
use crate::handler::balance_handler::{};

pub fn balance_routes() -> Router {
    Router::new()
        .route("/onramp", post(onramp))
        .route("/equity/available", get(get_available_equity))
}