use nolan::Commitlog;
use std::sync::Mutex;
//use napi::bindgen_prelude::BigInt;
use napi_derive::*;

use crate::message::JsMessage;

pub struct Producer {
    topic: String,
    commitlog: Mutex<Commitlog>,
}

impl Producer {
    pub fn new(
        directory: String,
        topic_name: String,
        max_segment_size_bytes: u64,
        max_commitlog_size: u64,
    ) -> Self {
        
        let cl = Commitlog::new(directory, max_segment_size_bytes, max_commitlog_size);
        let producer = Producer {
            topic: topic_name,
            commitlog: Mutex::new(cl),
        };
        producer
    }

}

#[napi(js_name = "Producer")]
pub struct JsProducer {
    producer: Producer
}

#[napi]
impl JsProducer {
    #[napi(constructor)]
    pub fn new() {}

    pub fn new_jsProducer_from_producer(producer: Producer) -> JsProducer{
        JsProducer { producer: producer }
    }

    #[napi]
    pub fn produce_message(&mut self, message: &mut JsMessage) {
        let message_bytes = message.serialize_message();
        let mut cl = self.producer.commitlog.lock().expect("lock has been poisoned...");
        cl.append(&message_bytes);
    }

    // #[napi]
    // pub fn produce_bytes(&mut self, bytes: Buffer) {
    //     let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
    //     cl.append(bytes);
    // }

    #[napi]
    pub fn get_topic(&self) -> String {
        self.producer.topic.clone()
    }
}
