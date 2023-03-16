use capnp::{serialize, message::ReaderOptions};
use log::{info, warn};

use crate::lucid_schema_capnp::{message_envelope};

pub fn parse_response(data: Vec<u8>) {
    // Deserializing object
   let reader = serialize::read_message(
        data.as_slice(),
        ReaderOptions::new()
    ).unwrap();

    let message_envelope = reader.get_root::<message_envelope::Reader>().unwrap();
    match message_envelope.which() {
        Ok(message_envelope::TopicResponse(envelope_topic_response)) => {
            let topic_response = envelope_topic_response.expect("Unable to get topic request from envelope");
            info!("Topic Response ------------");
            info!("Topic Name: {}", topic_response.get_topic_name().unwrap());
            info!("Status: {}", topic_response.get_success());
        },
        Ok(message_envelope::ProduceResponse(envelope_produce_response)) => {
            let produce_response = envelope_produce_response.expect("Unable to get produce request from envelope");
            info!("Produce Response ------------");
            info!("Topic Name: {}", produce_response.get_topic_name().unwrap());
            info!("Status: {}", produce_response.get_success());
            info!("Last offset: {}", produce_response.get_offset());
        },
        Ok(message_envelope::ConsumeResponse(envelope_consume_response)) => {
            let consume_response = envelope_consume_response.unwrap();
            info!("Consume Response ------------");
            info!("Topic Name: {}", consume_response.get_topic_name().unwrap());
            info!("Status: {}", consume_response.get_success());
            let messages = consume_response.get_messages().unwrap();
            for msg in messages {
                info!("Key: {:?}", msg.get_key().unwrap());
                info!("Value: {:?}", msg.get_value().unwrap());
                info!("Timestamp: {}", msg.get_timestamp());
            }
        },
        Ok(message_envelope::TopicRequest(envelope_topic_request)) => {
            let topic_request = envelope_topic_request.unwrap();
            warn!("{}", topic_request.get_topic_name().unwrap());
        },
        Ok(message_envelope::ConsumeRequest(envelope_produce_request)) => {
            warn!("{}", envelope_produce_request.unwrap().get_topic_name().unwrap());
        },
        Ok(message_envelope::ProduceRequest(envelope_consume_request)) => {
            warn!("{}", envelope_consume_request.unwrap().get_topic_name().unwrap());
        },
        Err(::capnp::NotInSchema(_)) => {
            info!("Unable to parse cap n p message");
        }
    }
}