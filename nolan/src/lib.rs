pub mod commitlog;
mod index;
mod segment;
use serde::{Deserialize, Serialize};
use std::str;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use commitlog::Commitlog;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(key: &[u8], value: &[u8], timestamp: i64) -> Message {
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        let message = Message {
            key: key_vec,
            value: value_vec,
            timestamp: timestamp,
        };
        return message;
    }

    pub fn new_without_timestamp(key: &[u8], value: &[u8]) -> Message {
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let message = Message {
            key: key_vec,
            value: value_vec,
            timestamp: timestamp,
        };
        return message;
    }

    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        return encoded_message;
    }

    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message = bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        return decoded_message;
    }
}

pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_offset: usize,
}

impl Consumer {
    pub fn new(topic: String) -> Consumer {
        let cl = Commitlog::new(topic.clone());

        let consumer = Consumer {
            topic: topic,
            commitlog: cl,
            consumer_offset: 0,
        };

        return consumer;
    }

    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {

        //Let's check if there are any new segments added.
        self.commitlog.reload_segments();

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> =  Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            match self.commitlog.read(self.consumer_offset) {
                Ok(buffer) => {
                    let thing = Message::deserialize_message(&buffer);
                    records.push(thing);
                    self.consumer_offset = self.consumer_offset + 1;
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        println!("Unable to get value for {}", self.consumer_offset);
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

    // pub fn consume(&mut self, offset: usize) {
    //     self.commitlog.read(offset);
    // }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}

pub struct Producer {
    topic: String,
    commitlog: Commitlog,
}

impl Producer {
    pub fn new(topic: String) -> Producer {
        let cl = Commitlog::new(topic.clone());

        let consumer = Producer {
            topic: topic,
            commitlog: cl,
        };

        return consumer;
    }

    pub fn produce(&mut self, message: &[u8]) {
        self.commitlog.append(message);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}
