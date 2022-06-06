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
    // #[new]
    // pub fn py_new(key: &[u8], value: &[u8], timestamp: i64) -> PyResult<Message> {
    //     let key_vec = key.to_vec();
    //     let value_vec = value.to_vec();
    //     let message = Message {
    //         key: key_vec,
    //         value: value_vec,
    //         timestamp: timestamp,
    //     };
    //     return message;
    // }
    #[new]
    pub fn py_new_without_timestamp(key: &[u8], value: &[u8]) -> Message {
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

    // pub fn deserialize_message(_py: Python, message_bytes: &PyBytes) -> Message {
    //     let decoded_message: Message = bincode::deserialize(message_bytes.as_bytes()).expect("Unable to deserialize message");
    //     return decoded_message;
    // }
}

//This should go above but it doesnt work lol
// pub fn deserialize_message(message_bytes: &[u8]) -> Message {
//     let decoded_message: Message = bincode::deserialize(message_bytes).expect("Unable to deserialize message");
//     return decoded_message;
// }