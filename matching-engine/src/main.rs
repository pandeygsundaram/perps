use crate::engine::start_processing;

mod dispatcher;
mod engine;
mod fills_processor;
mod markets;
#[allow(dead_code)]
mod message_flow;
mod publisher;
mod settlement;
mod types;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
    start_processing().await;
}
