use std::{sync::{Arc, Mutex}};
use crate::{message::Message, lucid::Topic};

pub struct Producer {
    topic: Arc<Mutex<Topic>>
}

impl Producer {
    pub fn new( producer_topic: Arc<Mutex<Topic>>) -> Producer {
        Producer {
            topic: producer_topic
        }
    }

    pub fn produce_message(&mut self, mut message: Message) -> u16 {
        let message_bytes = message.serialize_message();
        self.topic.lock().unwrap().commitlog.append(&message_bytes)
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) -> u16 {
        self.topic.lock().unwrap().commitlog.append(&bytes)
    }

    pub fn get_topic(&self) -> String {
        self.topic.lock().unwrap().name.clone()
    }
}
