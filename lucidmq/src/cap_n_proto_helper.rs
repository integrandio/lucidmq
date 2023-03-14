use capnp::message::{Builder, ReaderOptions, TypedReader, TypedBuilder};
use capnp::serialize::{self};
use log::info;

use crate::lucid_schema_capnp::{topic_response, produce_response, consume_response, message_envelope, topic_request, produce_request, consume_request};

use crate::types::{Command};

pub fn new_topic_response(topic_name: &str, is_success: bool) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name(topic_name);
    topic_response.set_success(is_success);
    topic_response.set_create(());

    message_envelope.set_topic_response(topic_response.reborrow_as_reader()).expect("Unable to set message");

    let serialized_message = serialize::write_message_to_words(&response_message_envelope);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

pub fn new_produce_response(topic_name: &str, last_offset: u64, is_success: bool) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut produce_response = request_message.init_root::<produce_response::Builder>();

    produce_response.set_topic_name(topic_name);
    produce_response.set_offset(last_offset);
    produce_response.set_success(is_success);

    message_envelope.set_produce_response(produce_response.reborrow_as_reader()).expect("Unable to set message");

    let serialized_message = serialize::write_message_to_words(&response_message_envelope);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

pub fn new_consume_response(topic_name: &str, is_success: bool) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut consume_reponse = request_message.init_root::<consume_response::Builder>();

    consume_reponse.set_success(is_success);
    consume_reponse.set_topic_name(topic_name);
    
    let mut messages = consume_reponse.init_messages(1);
    {
        let mut message_thing = messages.reborrow().get(0);
        message_thing.set_key("key".as_bytes());
        message_thing.set_value("value".as_bytes());
        message_thing.set_timestamp(1);
    }
    message_envelope.set_consume_response(request_message.get_root_as_reader().unwrap()).expect("Unable to set message");

    let serialized_message = serialize::write_message_to_words(&response_message_envelope);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

fn create_message_frame(mut original_message: Vec<u8>) -> Vec<u8> {
    let size_u16= u16::try_from(original_message.len()).unwrap();
    let thing = size_u16.to_le_bytes();
    // Append the size in bytes to the begining of the vector
    original_message.splice(0..0, thing.iter().cloned());
    return original_message;
}

pub fn parse_request(conn_id: String, data: Vec<u8>) -> Command {
    // Deserializing object
   let reader = serialize::read_message(
        data.as_slice(),
        ReaderOptions::new()
    ).unwrap();

    let message_envelope = reader.get_root::<message_envelope::Reader>().unwrap();
    match message_envelope.which() {
        Ok(message_envelope::TopicRequest(envelope_topic_request)) => {
            let topic_request = envelope_topic_request.expect("Unable to get topic request from envelope");
            // TODO: this does a copy, fix this to not do a copy
            let mut message = TypedBuilder::<topic_request::Owned>::new_default();
            message.set_root(topic_request).unwrap();
            let typed_reader = TypedReader::from(message);
            Command::TopicRequest { 
                conn_id: conn_id,
                capmessage: typed_reader
            }
        },
        Ok(message_envelope::ProduceRequest(envelope_produce_request)) => {
            let produce_request = envelope_produce_request.expect("Unable to get produce request from envelope");
            let _messages = produce_request.get_messages().expect("Unable to get message from produce request");
            for msg in _messages.into_iter() {
                info!("KEY: {:?}, VALUE: {:?}, TIMESTAMP: {}", msg.get_key().unwrap(), msg.get_value().unwrap(), msg.get_timestamp());
            }
            let topic_name = produce_request.get_topic_name().expect("Unable to get topic name from produce request");
            info!("{}", topic_name);
            let mut message = TypedBuilder::<produce_request::Owned>::new_default();
            message.set_root(produce_request).unwrap();
            let typed_reader = TypedReader::from(message);
            Command::ProduceRequest { 
                conn_id: conn_id,
                capmessage: typed_reader
            }
        },
        Ok(message_envelope::ConsumeRequest(envelope_consume_request)) => {
            let consume_request = envelope_consume_request.unwrap();
            let mut message = TypedBuilder::<consume_request::Owned>::new_default();
            message.set_root(consume_request).unwrap();
            let typed_reader = TypedReader::from(message);
            Command::ConsumeRequest { 
                conn_id: conn_id,
                capmessage: typed_reader
            }
        },
        Ok(message_envelope::TopicResponse(envelope_topic_response)) => {
            info!("{}", envelope_topic_response.unwrap().get_topic_name().unwrap());
            Command::Invalid { 
                message: "id1".to_string()
            }
        },
        Ok(message_envelope::ConsumeResponse(envelope_produce_response)) => {
            info!("{}", envelope_produce_response.unwrap().get_topic_name().unwrap());
            Command::Invalid { 
                message: "id1".to_string()
            }
        },
        Ok(message_envelope::ProduceResponse(envelope_consume_response)) => {
            info!("{}", envelope_consume_response.unwrap().get_topic_name().unwrap());
            Command::Invalid { 
                message: "id1".to_string()
            }
        },
        Err(::capnp::NotInSchema(_)) => {
            info!("Unable to parse cap n p message");
            Command::Invalid { 
                message: "id1".to_string()
            }
        }
    }
}