use capnp::message::{Builder, ReaderOptions, TypedReader, TypedBuilder};
use capnp::serialize::{self};
use log::{info};

use crate::lucid_schema_capnp::{topic_response, produce_response, consume_response, message_envelope, topic_request, produce_request, consume_request, message};

use crate::types::{Command};

pub fn new_topic_response_create(topic_name: &str, is_success: bool) -> Vec<u8> {
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


pub fn new_topic_response_describe(topic_name: &str, is_success: bool, max_retention: u64, max_segment: u64, consumer_groups: Vec<String>) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name(topic_name);
    
    if is_success && consumer_groups.len() > 0 {
        topic_response.set_success(is_success);
        let mut describe = topic_response.init_describe();
        describe.set_max_retention_bytes(max_retention);
        describe.set_max_segment_bytes(max_segment);
        //Builde our consumer group response
        let size = u32::try_from(consumer_groups.len()).unwrap();
        let mut cgs = describe.init_consumer_groups(size);
        for (i, consumer_group) in consumer_groups.iter().enumerate() {
            let ind_u32 = u32::try_from(i).unwrap();
            {
                cgs.reborrow().set(ind_u32, consumer_group);
            }
        }
    } else {
        topic_response.set_success(is_success);
        let mut describe = topic_response.init_describe();
        describe.set_max_retention_bytes(max_retention);
        describe.set_max_segment_bytes(max_segment);
        describe.init_consumer_groups(0);

    }
    message_envelope.set_topic_response(request_message.get_root_as_reader().unwrap()).expect("Unable to set message");

    let serialized_message = serialize::write_message_to_words(&response_message_envelope);
    let framed_message = create_message_frame(serialized_message);

    return framed_message;
}

pub fn new_topic_response_delete(topic_name: &str, is_success: bool) -> Vec<u8>{
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name(topic_name);
    topic_response.set_success(is_success);
    topic_response.set_delete(());

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

pub fn new_consume_response(topic_name: &str, is_success: bool, message_data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut consume_reponse = request_message.init_root::<consume_response::Builder>();

    consume_reponse.set_topic_name(topic_name);

    if is_success && message_data.len() > 0 {
        consume_reponse.set_success(is_success);
        let size = u32::try_from(message_data.len()).unwrap();
        let mut messages = consume_reponse.init_messages(size);

        for (i, msg) in message_data.iter().enumerate() {
            let message_reader = serialize::read_message(
                msg.as_slice(),
                ReaderOptions::new()
            ).unwrap();
            let reader = message_reader.get_root::<message::Reader>().unwrap();
            let thing = u32::try_from(i).unwrap();
            info!("CREATING LIST ind {} {:?}", thing, reader.get_key().unwrap());
            {
                messages.reborrow().set_with_caveats(thing, reader).unwrap();
            }
        }
    } else {
        consume_reponse.set_success(false);
        consume_reponse.init_messages(0);
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