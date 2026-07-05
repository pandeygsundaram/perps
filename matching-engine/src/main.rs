use crate::engine::start_processing;

mod dispatcher;
mod engine;
mod fills_processor;
mod markets;
mod publisher;
mod settlement;
mod types;
mod test;

#[tokio::main]
async fn main() {
    start_processing().await;
}
