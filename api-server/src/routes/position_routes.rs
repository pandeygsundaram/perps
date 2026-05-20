use axum::{routing::get, Router,};
use crate::handler::auth_handler::{};

pub fn position_routes() -> Router {
    Router::new()
        .route("/positions/open/{marketId}", get(get_open_position))
        .route("/signup", get(get_closed_positions))
}