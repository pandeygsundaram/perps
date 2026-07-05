use axum::{routing::post,Router,};


pub fn balance_routes() -> Router {
    Router::new()
        .route("/onramp", post(onramp))
        .route("/equity/available", get(get_available_equity))
}