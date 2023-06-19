use crate::{topic::Topic, lucidmq_errors::ProducerError};
use log::{error};
use std::sync::{Arc, RwLock};

pub struct Producer {
    topic: Arc<RwLock<Topic>>,
}

impl Producer {
    pub fn new(producer_topic: Arc<RwLock<Topic>>) -> Producer {
        Producer {
            topic: producer_topic,
        }
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) -> Result<u16, ProducerError> {
        let written_offset = self.topic.write().unwrap().commitlog.append(&bytes).map_err(|e| {
            error!("{}", e);
            ProducerError::new("Oldest offset does not map to a u32")
        })?;
        Ok(written_offset)
    }

    pub fn _produce_bytes_vector(&mut self, bytes_vector: Vec<&[u8]>) -> u16 {
        let mut last_offset = 0;
        let commitlog = &mut self.topic.write().unwrap().commitlog;
        for bytes in bytes_vector {
            last_offset = commitlog.append(&bytes).expect("Unable to produce messsage to commitlog");
        }
        last_offset
    }

    pub fn _get_topic(&self) -> String {
        self.topic.read().unwrap().name.clone()
    }
}
