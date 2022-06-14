use nolan::Commitlog;
use std::sync::Mutex;

use crate::message::Message;

pub struct Producer {
    topic: String,
    commitlog: Mutex<Commitlog>,
}

impl Producer {
    pub fn new(directory: String, topic: String) -> Producer {
        let cl = Commitlog::new(directory.clone());
        let consumer = Producer {
            topic: topic,
            commitlog: Mutex::new(cl),
        };

        return consumer;
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) {
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(bytes);
    }

    pub fn produce_message(&mut self, mut message: Message) {
        let message_bytes = message.serialize_message();
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(&message_bytes);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}
