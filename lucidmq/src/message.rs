use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(key: &[u8], value: &[u8], timestamp: Option<i64>) -> Message {
        let message_timestamp: i64;
        match timestamp {
            Some(timestamp) => {
                message_timestamp = timestamp;
            }
            None => {
                message_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;
            }
        }
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        let message = Message {
            key: key_vec,
            value: value_vec,
            timestamp: message_timestamp,
        };
        return message;
    }

    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        return encoded_message;
    }

    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message =
            bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        return decoded_message;
    }
}