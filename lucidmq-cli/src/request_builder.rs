use capnp::message::Builder;
use capnp::serialize;

use crate::lucid_schema_capnp::{topic_request, produce_request, consume_request};


pub fn new_topic_request() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut topic_request = request_message.init_root::<topic_request::Builder>();

    topic_request.set_topic_name("topic1");
    topic_request.set_create(());

    let data = serialize::write_message_to_words(&request_message);
    data
}

pub fn new_produce_request() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut produce_request = request_message.init_root::<produce_request::Builder>();

    produce_request.set_topic_name("topic1");

    let mut messages = produce_request.init_messages(1);
    {
        let mut message_thing = messages.reborrow().get(0);
        message_thing.set_key("key".as_bytes());
        message_thing.set_value("value".as_bytes());
        message_thing.set_timestamp(1);
    }

    let data = serialize::write_message_to_words(&request_message);
    data
}

pub fn new_consume_message() -> Vec<u8> {
    let mut request_message = Builder::new_default();
    let mut consume_request = request_message.init_root::<consume_request::Builder>();

    consume_request.set_topic_name("topic1");
    consume_request.set_consumer_group("cg1");
    consume_request.set_timout(1);

    let data = serialize::write_message_to_words(&request_message);
    data
}