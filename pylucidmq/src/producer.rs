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
        let consumer = Producer {
            topic: topic,
            commitlog: Mutex::new(cl),
        };

        return consumer;
    }

    pub fn produce_message(&mut self, mut message: Message) {
        let vector_thing = message.serialize_message();
        let vec_point = &vector_thing;
        let c: &[u8] = &vec_point;
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(c);
    }

    pub fn produce(&mut self, message: &[u8]) {
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(message);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}