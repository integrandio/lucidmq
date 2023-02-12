mod topic;
mod broker;
mod message;
mod consumer;
mod types;
mod response_builder;
mod server;
pub mod lucid_schema_capnp;

use env_logger::Builder;
use log::LevelFilter;
use log::{info};
use tokio::sync::mpsc;
use types::{SenderType, RecieverType};



#[tokio::main]
pub async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    info!("Starting lucidmq");
    let request_channel_sender: SenderType;
    let request_channel_reciever: RecieverType;
    (request_channel_sender, request_channel_reciever) = mpsc::channel(32);
    let response_channel_sender: SenderType;
    let response_channel_reciever: RecieverType;
    (response_channel_sender, response_channel_reciever) = mpsc::channel(32);

    let broker = broker::Broker::new("test_log".to_string(), 100, 100);
    tokio::spawn(async move {
        broker.run(request_channel_reciever, response_channel_sender).await;
    });
    let server = server::LucidQuicServer::new(
        request_channel_sender,
        response_channel_reciever
    );

    server.run_server().await;
}