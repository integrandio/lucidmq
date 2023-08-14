use std::fmt;

use capnp::message::{TypedReader, Builder, HeapAllocator};
use tokio::sync::mpsc::{Sender, Receiver};
use crate::lucid_schema_capnp::{produce_request, topic_request, consume_request};

pub enum Command{
    TopicRequest {
        conn_id: String,
        capmessage: TypedReader::<Builder<HeapAllocator>, topic_request::Owned>

    },
    ProduceRequest {
        conn_id: String,
        capmessage: TypedReader::<Builder<HeapAllocator>, produce_request::Owned>

    },
    ConsumeRequest {
        conn_id: String,
        capmessage: TypedReader::<Builder<HeapAllocator>, consume_request::Owned>

    },
    Response {
        conn_id: String,
        capmessagedata: Vec<u8>
    },
    Invalid {
        conn_id: String,
        error_message: String,
        capmessage_data: Vec<u8>
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Command::TopicRequest { conn_id, capmessage: _ } => {
                f.debug_struct("Command")
                .field("Command Type", &"TopicRequest")
                .field("Connection ID", &conn_id)
                .finish()
            },
            Command::ProduceRequest { conn_id, capmessage: _ } => {
                f.debug_struct("Command")
                .field("Command Type", &"ProduceRequest")
                .field("Connection ID", &conn_id)
                .finish()
            },
            Command::ConsumeRequest { conn_id, capmessage: _ } => {
                f.debug_struct("Command")
                .field("Command Type", &"ConsumeRequest")
                .field("Connection ID", &conn_id)
                .finish()
            },
            Command::Response { conn_id, capmessagedata: _ } => {
                f.debug_struct("Command")
                .field("Command Type", &"Response")
                .field("Connection ID", &conn_id)
                .finish()
            },
            Command::Invalid { conn_id, error_message, capmessage_data: _} => {
                f.debug_struct("Command")
                .field("Command Type", &"Invalid")
                .field("Connection ID", &conn_id)
                .field("Error Message", &error_message)
                .finish()
            },
        }

    }
}

//For using command with mpsc
pub type SenderType = Sender<Command>;
pub type RecieverType = Receiver<Command>;