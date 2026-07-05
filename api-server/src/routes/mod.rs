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

// what api endpoints are pending right now??
// we'll simply wire things up from here on now!
// so what are we gonna do is that 
// so the thing is that umm what you have to do is that
// /login , /signup , is there 
// /onramp, /equity/available (get balance) is there
// /put order to modify existing order type shit
// /some things we are reading from the backend and some from the engine
// engine is it ready??
// don't think so
// we lock in on the thread thingy now and will wrap it up!