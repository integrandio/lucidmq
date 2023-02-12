use tokio::sync::mpsc::{Sender, Receiver};


#[derive(Debug)]
pub struct Payload {
    pub conn_id: String,
    pub message: String,
    pub data: Vec<u8>

}

#[derive(Debug)]
pub enum Command{ 
    Produce (Payload),
    Consume (Payload),
    Topic (Payload),
    Response  (Payload),
    Invalid {
        message: String
    }
}

//For using command with mpsc
pub type SenderType = Sender<Command>;
pub type RecieverType = Receiver<Command>;