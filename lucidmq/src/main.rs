mod broker;
mod cap_n_proto_helper;
mod consumer;
pub mod lucid_schema_capnp;
mod producer;
mod lucidmq_errors;
mod tcp_server;
mod topic;
mod types;

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

    let broker = broker::Broker::new("test_log".to_string());
    tokio::spawn(async move {
        broker
            .run(request_channel_reciever, response_channel_sender)
            .await;
    });
    let server = tcp_server::LucidTcpServer::new(request_channel_sender, response_channel_reciever);
    // let server = quic_server::LucidQuicServer::new(request_channel_sender, response_channel_reciever);
    server.run_server().await;
}
