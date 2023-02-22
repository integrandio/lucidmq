use capnp::message::{Builder, ReaderOptions};
use capnp::serialize;
use log::info;

use crate::lucid_schema_capnp::{topic_response, produce_response, consume_response, message_envelope, topic_request};


pub fn new_topic_response() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name("topic1");
    topic_response.set_success(true);
    topic_response.set_create(());

    let serialized_message = serialize::write_message_to_words(&request_message);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

pub fn new_produce_response() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut produce_response = request_message.init_root::<produce_response::Builder>();

    produce_response.set_topic_name("topic1");
    produce_response.set_offset(0);

    let serialized_message = serialize::write_message_to_words(&request_message);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
    
}

pub fn new_consume_response() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut consume_reponse = request_message.init_root::<consume_response::Builder>();

    consume_reponse.set_success(true);
    let mut messages = consume_reponse.init_messages(1);
    {
        let mut message_thing = messages.reborrow().get(0);
        message_thing.set_key("key".as_bytes());
        message_thing.set_value("value".as_bytes());
        message_thing.set_timestamp(1);
    }

    let serialized_message = serialize::write_message_to_words(&request_message);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

fn create_message_frame(mut original_message: Vec<u8>) -> Vec<u8> {
    info!("Og message size: {}", original_message.len());
    info!("{:?}", original_message);

    let size_u16= u16::try_from(original_message.len()).unwrap();
    let thing = size_u16.to_le_bytes();
    info!("Size of thing {}", thing.len());
    info!("{:?}", thing);

    // Append the size in bytes to the begining of the vector
    original_message.splice(0..0, thing.iter().cloned());

    info!("new message size: {}", original_message.len());
    info!("{:?}", original_message);

    return original_message;
}

pub fn parse_cap_message(data: Vec<u8>) {
    // Deserializing object
   let reader = serialize::read_message(
        data.as_slice(),
        ReaderOptions::new()
        ).unwrap();

    let message_envelope = reader.get_root::<message_envelope::Reader>().unwrap();
    if message_envelope.has_topic_request() {
        let topic_request = reader.get_root::<topic_request::Reader>().unwrap();
        let topic_name = topic_request.get_topic_name().unwrap();
        info!("{}", topic_name)
    }
}