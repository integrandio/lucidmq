use pyo3::{prelude::*};
use nolan::Commitlog;
use std::str;
use std::sync::Mutex;
use crate::Message;

#[pyclass]
pub struct Producer {
    topic: String,
    commitlog: Mutex<Commitlog>,
}

#[pymethods]
impl Producer {
    #[new]
    pub fn new(directory: String, topic: String) -> Producer {
        let cl = Commitlog::new(directory.clone());
        Producer {
            topic: topic,
            commitlog: Mutex::new(cl),
        }
    }

    pub fn produce_message(&mut self, mut message: Message) {
        let message_bytes = message.serialize_message();
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(&message_bytes);
    }

    pub fn produce_bytes(&mut self, message: &[u8]) {
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(message);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}