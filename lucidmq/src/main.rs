mod topic;
mod server;
mod broker;
mod message;
mod consumer;
use std::sync::Arc;
use env_logger::Builder;
use log::LevelFilter;
use log::{info};

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};

pub type SenderType = Sender<Command>;
pub type RecieverType = Receiver<Command>;


#[derive(Debug)]
pub enum Command{ 
    Produce {
        key: String,
    },
    Consume {
        key: String,
    },
    Topic {
        key: String
    },
    Response  {
        key: String
    },
    Invalid {
        key: String
    }
}

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

    let server = server::LucidServer::new(
        request_channel_sender,
        response_channel_reciever
    );

    let broker = broker::Broker::new("test_log".to_string(), 100, 100);
    tokio::spawn(async move {
        broker.run(request_channel_reciever, response_channel_sender).await;
    });
    let _res = server.start().await;
}