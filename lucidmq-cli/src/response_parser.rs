use capnp::{serialize, message::ReaderOptions};

use crate::lucid_schema_capnp::{topic_response, produce_response, consume_response};

pub fn read_message_from_bytes(message_bytes: Vec<u8>) {
    let reader = serialize::read_message(message_bytes.as_slice(), ReaderOptions::new()).unwrap();
    let produce_response = reader.get_root::<produce_response::Reader>().unwrap();

    let topic_name = produce_response.get_topic_name().unwrap();
    let offset = produce_response.get_offset();

    println!("Topic name: {} offset: {}", topic_name, offset);
}