use capnp::message::Builder;
use capnp::serialize;

use crate::lucid_schema_capnp::{topic_request, produce_request, consume_request, message_envelope};


pub fn new_topic_request() -> Vec<u8> {
    let mut request_message_envelope = Builder::new_default();
    let mut message_envelope = request_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_request = request_message.init_root::<topic_request::Builder>();

    topic_request.set_topic_name("topic1");
    topic_request.set_create(());

    message_envelope.set_topic_request(topic_request.reborrow_as_reader()).expect("Unable to set message sent");

    let serialized_message = serialize::write_message_to_words(&request_message_envelope);
    let framed_message = create_message_frame(serialized_message);
    framed_message
}

pub fn new_produce_request() -> Vec<u8> {
    let mut request_message_envelope = Builder::new_default();
    let mut message_envelope = request_message_envelope.init_root::<message_envelope::Builder>();
    let mut request_message = Builder::new_default();
    {
        let mut produce_request = request_message.init_root::<produce_request::Builder>();
    
        produce_request.set_topic_name("topic1");
    
        let mut messages = produce_request.init_messages(1);
        {
            let mut message_thing = messages.reborrow().get(0);
            message_thing.set_key("key".as_bytes());
            message_thing.set_value("value".as_bytes());
            message_thing.set_timestamp(1);
        }
    }

    message_envelope.set_produce_request(request_message.get_root_as_reader().expect("unable to get reader")).expect("Unable to set message sent");

    let serialized_message = serialize::write_message_to_words(&request_message_envelope);
    let framed_message = create_message_frame(serialized_message);
    framed_message
}

pub fn new_consume_message() -> Vec<u8> {
    let mut request_message_envelope = Builder::new_default();
    let mut message_envelope = request_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut consume_request = request_message.init_root::<consume_request::Builder>();

    consume_request.set_topic_name("topic1");
    consume_request.set_consumer_group("cg1");
    consume_request.set_timout(1);


    message_envelope.set_consume_request(consume_request.reborrow_as_reader()).expect("Unable to set message sent");

    let serialized_message = serialize::write_message_to_words(&request_message_envelope);
    let framed_message = create_message_frame(serialized_message);
    framed_message
}


fn create_message_frame(mut original_message: Vec<u8>) -> Vec<u8> {
    let size_u16= u16::try_from(original_message.len()).unwrap();
    let thing = size_u16.to_le_bytes();
    // Append the size in bytes to the begining of the vector
    original_message.splice(0..0, thing.iter().cloned());
    return original_message;
}