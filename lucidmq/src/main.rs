mod broker;
mod cap_n_proto_helper;
mod consumer;
pub mod lucid_schema_capnp;
mod producer;
mod lucidmq_errors;
mod tcp_server;
mod topic;
mod types;

use std::env;

use env_logger::Builder;
use log::info;
use log::LevelFilter;
use tokio::sync::mpsc;
use types::{RecieverType, SenderType};

#[tokio::main]
pub async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    info!("Starting lucidmq");
    // Initialize all of our channels
    let request_channel_sender: SenderType;
    let request_channel_reciever: RecieverType;
    (request_channel_sender, request_channel_reciever) = mpsc::channel(32);
    let response_channel_sender: SenderType;
    let response_channel_reciever: RecieverType;
    (response_channel_sender, response_channel_reciever) = mpsc::channel(32);

    let host = get_env_variable("HOST", "127.0.0.1");
    let port = get_env_variable("PORT", "6969");
    let lucidmq_directory = get_env_variable("LUCIDMQ_DIRECTORY", "test_log");

    let broker = broker::Broker::new(lucidmq_directory).unwrap();
    tokio::spawn(async move {
        broker
            .run(request_channel_reciever, response_channel_sender)
            .await;
    });
    let server = tcp_server::LucidTcpServer::new(
        &host,
        &port,
        request_channel_sender,
        response_channel_reciever).unwrap();
    server.run_server().await;
}

fn get_env_variable(variable_name: &str, fallback: &str) -> String {
    match env::var(variable_name) {
        Ok(v) => v,
        Err(_) => fallback.to_string()
    }
}