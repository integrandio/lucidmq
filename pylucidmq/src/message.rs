use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

#[pyclass]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
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
    pub fn new(message_key: &[u8], message_value: &[u8]) -> Message {
        let key_vec = message_key.to_vec();
        let value_vec = message_value.to_vec();
        let message_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        Message {
            key: key_vec,
            value: value_vec,
            timestamp: message_timestamp,
        }
    }

    #[staticmethod]
    pub fn new_with_timestamp(
        message_key: &[u8],
        message_value: &[u8],
        message_timestamp: i64,
    ) -> Message {
        let key_vec = message_key.to_vec();
        let value_vec = message_value.to_vec();
        Message {
            key: key_vec,
            value: value_vec,
            timestamp: message_timestamp,
        }
    }

    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        encoded_message
    }

    #[staticmethod]
    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message =
            bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        decoded_message
    }
}
