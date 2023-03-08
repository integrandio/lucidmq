use crate::topic::{Topic};
use std::{sync::{Arc, RwLock}};

pub struct Producer {
    topic: Arc<RwLock<Topic>>
}

impl Producer {
    pub fn new( producer_topic: Arc<RwLock<Topic>>) -> Producer {
        Producer {
            topic: producer_topic
        }
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) -> u16 {
        self.topic.write().unwrap().commitlog.append(&bytes)
        //self.topic.lock().unwrap().commitlog.append(&bytes)
    }

    pub fn _produce_bytes_vector(&mut self, bytes_vector: Vec<&[u8]>) -> u16 {
        let mut last_offset = 0;
        let commitlog = &mut self.topic.write().unwrap().commitlog;
        for bytes in bytes_vector {
            last_offset = commitlog.append(&bytes)
        }
        last_offset
    }

    pub fn _get_topic(&self) -> String {
        self.topic.read().unwrap().name.clone()
    }
}
