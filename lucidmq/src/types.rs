use tokio::sync::mpsc::{Sender, Receiver};

//Commands for passing messages between broker and server
#[derive(Debug)]
pub enum Command{ 
    Produce {
        conn_id: String,
        key: String,
        data: Vec<u8>
    },
    Consume {
        conn_id: String,
        key: String,
        data: Vec<u8>
    },
    Topic {
        conn_id: String,
        key: String,
        topic_name: String,
    },
    Response  {
        conn_id: String,
        key: String,
        data: Vec<u8>
    },
    Invalid {
        key: String
    }
}

//For using command with mpsc
pub type SenderType = Sender<Command>;
pub type RecieverType = Receiver<Command>;