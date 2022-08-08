use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use napi::bindgen_prelude::Buffer;
use napi_derive::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(key: &[u8], value: &[u8], timestamp: Option<i64>) -> Message {
        let message_timestamp: i64 = match timestamp {
            Some(timestamp) => timestamp,
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        };
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
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

    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message =
            bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        decoded_message
    }
}

#[napi(js_name = "Message")]
pub struct JsMessage {
    message: Message
}

#[napi]
impl JsMessage {
    #[napi(constructor)]
    pub fn new(key: Buffer, value: Buffer, timestamp: Option<i64>) -> Self {
        let message = Message::new(&key.to_vec(), &value.to_vec(), timestamp);
        JsMessage { message: message }
    }

    pub fn new_jsMessage_from_message(message: Message) -> JsMessage{
        JsMessage { message: message }
    }

    #[napi]
    pub fn serialize_message(&mut self) -> Buffer {
        let encoded_message: Vec<u8> = bincode::serialize(&self.message).expect("Unable to encode message");
        encoded_message.into()
    }

    #[napi]
    pub fn deserialize_message(message_bytes: Buffer) -> Self {
        let decoded_message: Message =
            bincode::deserialize(&message_bytes).expect("Unable to deserialize message");
        JsMessage { message: decoded_message }
    }

    #[napi]
    pub fn get_key(&self) -> Buffer{
        self.message.key.clone().into()
    }

    #[napi]
    pub fn get_value(&self) -> Buffer{
        self.message.value.clone().into()
    }

    #[napi]
    pub fn get_timestamp(&self) -> i64{
        self.message.timestamp.into()
    }
}
