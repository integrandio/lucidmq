use crate::lucid_schema_capnp::{
    consume_request, consume_response, message, message_envelope, produce_request,
    produce_response, topic_request, topic_response, invalid_response,
};
use crate::lucidmq_errors::ProtocolError;
use crate::topic::SimpleTopic;
use crate::types::Command;
use capnp::message::{Builder, ReaderOptions, TypedBuilder, TypedReader};
use capnp::{serialize, serialize_packed};
use log::{error, info, warn};

pub fn new_topic_response_create(topic_name: &str, is_success: bool) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name(topic_name);
    topic_response.set_success(is_success);
    topic_response.set_create(());

    message_envelope
        .set_topic_response(topic_response.reborrow_as_reader())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;
}

pub fn new_topic_response_describe(
    topic_name: &str,
    is_success: bool,
    max_retention: u64,
    max_segment: u64,
    consumer_groups: Vec<String>,
) -> Vec<u8> {
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
        //Build our consumer group response
        let size = u32::try_from(consumer_groups.len()).unwrap();
        let mut cgs: capnp::text_list::Builder<'_> = describe.init_consumer_groups(size);
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
    message_envelope
        .set_topic_response(request_message.get_root_as_reader().unwrap())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;
}

pub fn new_topic_response_all(is_success: bool, topics_datas: Vec<SimpleTopic>) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name("placeholder");
    topic_response.set_success(is_success);
    if is_success {
        let size = u32::try_from(topics_datas.len()).unwrap();
        let mut all = topic_response.init_all(size);
        for (i, topic_data) in topics_datas.iter().enumerate() {
            let index_u32 = u32::try_from(i).unwrap();
            let mut topics_list_object = all.reborrow().get(index_u32);
            topics_list_object.set_topic_name(&topic_data.topic_name);
            let consumer_group_size = u32::try_from(topic_data.consumer_groups.len()).unwrap();
            let mut cgs = topics_list_object.init_consumer_groups(consumer_group_size);
            for (j, consumer_group) in topic_data.consumer_groups.iter().enumerate() {
                let cg_ind_u32 = u32::try_from(j).unwrap();
                {
                    cgs.reborrow().set(cg_ind_u32, consumer_group);
                }
            }
        }
    }

    message_envelope
        .set_topic_response(request_message.get_root_as_reader().unwrap())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;
}

pub fn new_topic_response_delete(topic_name: &str, is_success: bool) -> Vec<u8> {
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut topic_response = request_message.init_root::<topic_response::Builder>();

    topic_response.set_topic_name(topic_name);
    topic_response.set_success(is_success);
    topic_response.set_delete(());

    message_envelope
        .set_topic_response(topic_response.reborrow_as_reader())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

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

    message_envelope
        .set_produce_response(produce_response.reborrow_as_reader())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;
}

pub fn new_consume_response(
    topic_name: &str,
    is_success: bool,
    message_data: Vec<Vec<u8>>,
) -> Vec<u8> {
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
            let message_reader =
                serialize::read_message(msg.as_slice(), ReaderOptions::new()).unwrap();
            let reader = message_reader.get_root::<message::Reader>().unwrap();
            let message_index = u32::try_from(i).unwrap();
            {
                messages
                    .reborrow()
                    .set_with_caveats(message_index, reader)
                    .unwrap();
            }
        }
    } else {
        consume_reponse.set_success(false);
        consume_reponse.init_messages(0);
    }
    message_envelope
        .set_consume_response(request_message.get_root_as_reader().unwrap())
        .expect("Unable to set message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;
}

pub fn new_invalid_response(message_text: &str) -> Vec<u8>{
    let mut response_message_envelope = Builder::new_default();
    let mut message_envelope = response_message_envelope.init_root::<message_envelope::Builder>();

    let mut request_message = Builder::new_default();
    let mut invalid_reponse = request_message.init_root::<invalid_response::Builder>();

    invalid_reponse.set_error_message(message_text);

    message_envelope.set_invalid_response(invalid_reponse.reborrow_as_reader()).expect("unable to set envelope message");

    let mut buffer = vec![];
    serialize_packed::write_message(&mut buffer, &response_message_envelope)
        .expect("Unable to serialize packed message");
    let framed_message = create_message_frame(buffer).unwrap();

    return framed_message;

}

fn create_message_frame(mut original_message: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
    // Convert to a u16 so that we have a byte representation of only 2 digits?
    let size_u16 = u16::try_from(original_message.len()).map_err(|e| {
        error!("Error when converting message size to u16 {}", e);
        ProtocolError::new("Unable to convert message size to u16")
    })?;
    let thing = size_u16.to_le_bytes();
    // Append the size in bytes to the begining of the vector
    original_message.splice(0..0, thing.iter().cloned());
    Ok(original_message)
}

pub fn parse_request(conn_id: String, data: Vec<u8>) -> Result<Command, capnp::Error> {
    // Deserializing object
    let reader = match serialize_packed::read_message(data.as_slice(), ReaderOptions::new()) {
        Ok(read) => read,
        Err(err) => {
            warn!("Error when attempting to read message from stream[capnperror {}]", err);
            return Ok(
                Command::Invalid { 
                    conn_id: conn_id,
                    error_message: "invalid message sent".to_string(),
                    capmessage_data: Vec::new()
                })
        }
    };
    let message_envelope = reader.get_root::<message_envelope::Reader>()?;
    match message_envelope.which() {
        Ok(message_envelope::TopicRequest(envelope_topic_request)) => {
            let topic_request =
                envelope_topic_request?;
            // TODO: this does a copy, fix this to not do a copy
            let mut message = TypedBuilder::<topic_request::Owned>::new_default();
            message.set_root(topic_request)?;
            let typed_reader = TypedReader::from(message);
            Ok(Command::TopicRequest {
                conn_id: conn_id,
                capmessage: typed_reader,
            })
        }
        Ok(message_envelope::ProduceRequest(envelope_produce_request)) => {
            let produce_request =
                envelope_produce_request.expect("Unable to get produce request from envelope");
            let _messages = produce_request
                .get_messages()?;
            // for msg in _messages.into_iter() {
            //     info!("KEY: {:?}, VALUE: {:?}, TIMESTAMP: {}", msg.get_key().unwrap(), msg.get_value().unwrap(), msg.get_timestamp());
            // }
            //let topic_name = produce_request.get_topic_name().expect("Unable to get topic name from produce request");
            // info!("{}", topic_name);
            let mut message = TypedBuilder::<produce_request::Owned>::new_default();
            message.set_root(produce_request)?;
            let typed_reader = TypedReader::from(message);
            Ok(Command::ProduceRequest {
                conn_id: conn_id,
                capmessage: typed_reader,
            })
        }
        Ok(message_envelope::ConsumeRequest(envelope_consume_request)) => {
            let consume_request = envelope_consume_request?;
            let mut message = TypedBuilder::<consume_request::Owned>::new_default();
            message.set_root(consume_request)?;
            let typed_reader = TypedReader::from(message);
            Ok(Command::ConsumeRequest {
                conn_id: conn_id,
                capmessage: typed_reader,
            })
        }
        Ok(message_envelope::TopicResponse(envelope_topic_response)) => {
            info!("{}", envelope_topic_response?.get_topic_name()?);
            Ok(Command::Invalid { 
                conn_id: conn_id,
                error_message: "Topic Response is an invalid request".to_string(),
                capmessage_data: Vec::new()
            })
        }
        Ok(message_envelope::ConsumeResponse(envelope_consume_response)) => {
            info!("{}",envelope_consume_response?.get_topic_name()?);
            Ok(Command::Invalid { 
                conn_id: conn_id,
                error_message: "Consume response is an invalid request".to_string(),
                capmessage_data: Vec::new()
            })
        }
        Ok(message_envelope::ProduceResponse(envelope_produce_response)) => {
            info!("{}", envelope_produce_response?.get_topic_name()?);
            Ok(Command::Invalid { 
                conn_id: conn_id,
                error_message: "Produce response is an invalid request".to_string(),
                capmessage_data: Vec::new()
            })
        }
        Ok(message_envelope::InvalidResponse(envelope_invalid_response)) => {
            info!("{}", envelope_invalid_response?.get_error_message()?);
            Ok(Command::Invalid { 
                conn_id: conn_id,
                error_message: "Invalid response is an invalid request".to_string(),
                capmessage_data: Vec::new()
            })
        }
        Err(::capnp::NotInSchema(_)) => {
            info!("Unable to parse cap n p message");
            Ok(Command::Invalid { 
                conn_id: conn_id,
                error_message: "Not in schema".to_string(),
                capmessage_data: Vec::new()
            })
        }
    }
}
