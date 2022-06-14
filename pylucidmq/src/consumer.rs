
use pyo3::{prelude::*};
use nolan::Commitlog;
use std::str;
use std::thread;
use std::time::{Duration, Instant};

use crate::Message;

#[pyclass]
pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_offset: usize,
}

#[pymethods]
impl Consumer {
    #[new]
    pub fn new(directory: String, topic: String) -> Consumer {
        let mut cl = Commitlog::new(directory.clone());
        let offset = &cl.get_oldest_offset();
        let consumer = Consumer {
            topic: topic,
            commitlog: cl,
            consumer_offset: *offset,
        };

        return consumer;
    }

    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {
        //Let's check if there are any new segments added.
        self.commitlog.reload_segments();

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            match self.commitlog.read(self.consumer_offset) {
                Ok(buffer) => {
                    let thing = deserialize_message(&buffer);
                    records.push(thing);
                    self.consumer_offset = self.consumer_offset + 1;
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        //println!("Unable to get value for {}", self.consumer_offset);
                        self.commitlog.reload_segments();
                        thread::sleep(ten_millis);
                        elapsed_duration = start_time.elapsed();
                    } else {
                        panic!("Unexpected error found")
                    }
                }
            };
        }
        return records;
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }

    pub fn get_offset(&self) -> usize {
        return self.consumer_offset;
    }
}

//This should go in the message crate but it doesnt work
pub fn deserialize_message(message_bytes: &[u8]) -> Message {
    let decoded_message: Message = bincode::deserialize(message_bytes).expect("Unable to deserialize message");
    return decoded_message;
}