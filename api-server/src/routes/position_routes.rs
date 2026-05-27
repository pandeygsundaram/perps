use axum::{routing::get, Router,};
use crate::handler::auth_handler::{};

pub fn position_routes() -> Router {
    Router::new()
        .route("/positions/open/{marketId}", get(get_open_position))
        .route("/positions/closed/{marketId}", get(get_closed_position))
}