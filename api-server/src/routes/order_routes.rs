use axum::{Router, routing::{get ,delete, post}};
use crate::handler::auth_handler::{};

pub fn order_routes() -> Router {
    Router::new()
        .route("/order", post(create_order))
        .route("/order", delete(delete_order))
        .route("/orders/open/{markerId}", get(get_open_order))
        .route("/orders/{marketId}", get(get_orders))
        .route("/fills",get(get_fills))
}