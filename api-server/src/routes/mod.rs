use axum::Router;
use sqlx::PgPool;

mod auth_routes;
mod balance_routes;
mod order_routes;
mod position_routes;

use self::{
    auth_routes::auth_routes, balance_routes::balance_routes, order_routes::order_routes,
    position_routes::position_routes,
};

pub fn app_routes() -> Router<PgPool> {
    let public_routes = Router::new().merge(auth_routes());

    let protected_routes = Router::new()
        .merge(balance_routes())
        .merge(order_routes())
        .merge(position_routes());

    Router::new().merge(public_routes).merge(protected_routes)
}
