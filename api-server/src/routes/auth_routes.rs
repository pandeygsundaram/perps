use axum::{Router, routing::{get, post}};
use crate::handler::auth_handler::{login_handler,signup_handler};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
}