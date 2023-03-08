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
        message: String
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command").finish()
    }
}

//For using command with mpsc
pub type SenderType = Sender<Command>;
pub type RecieverType = Receiver<Command>;