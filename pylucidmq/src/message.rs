use pyo3::{prelude::*};
use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

#[pyclass]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message{
    #[pyo3(get, set)]
    key: Vec<u8>,
    #[pyo3(get, set)]
    value: Vec<u8>,
    #[pyo3(get, set)]
    timestamp: i64,
}

#[pymethods]
impl Message {
    #[new]
    pub fn new(key: &[u8], value: &[u8]) -> Message {
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

    #[staticmethod]
    pub fn new_with_timestamp(key: &[u8], value: &[u8], timestamp: i64) -> Message {
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        Message {
            key: key_vec,
            value: value_vec,
            timestamp: timestamp,
        }
    }

    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        return encoded_message;
    }

    #[staticmethod]
    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message = bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        return decoded_message;
    }
}