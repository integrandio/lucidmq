use capnp::{serialize, message::ReaderOptions};
use std::io::Write;
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
            match topic_response.which() {
                Ok(crate::lucid_schema_capnp::topic_response::Which::Create(_create)) => {
                    write!(std::io::stdout(),"Topic Create Response ------------\n").unwrap();
                    write!(std::io::stdout(),"Topic Name: {}\n", topic_response.get_topic_name().unwrap()).unwrap();
                    write!(std::io::stdout(),"Status: {}\n", topic_response.get_success()).unwrap();
                    std::io::stdout().flush().unwrap();
                },
                Ok(crate::lucid_schema_capnp::topic_response::Which::Describe(describe)) => {
                    write!(std::io::stdout(),"Topic Describe Response ------------\n").unwrap();
                    write!(std::io::stdout(),"Topic Name: {}\n", topic_response.get_topic_name().unwrap()).unwrap();
                    write!(std::io::stdout(),"Status: {}\n", topic_response.get_success()).unwrap();
                    let cgs = describe.get_consumer_groups().unwrap();
                    let mut cgs_vec = Vec::new();
                    for msg in cgs {
                        cgs_vec.push(msg.unwrap().to_string())
                    }
                    write!(std::io::stdout(),"Topic max retention bytes: {}, max segments bytes: {}, consumer groups: {:?}\n", describe.get_max_retention_bytes(), describe.get_max_segment_bytes(), cgs_vec).unwrap();
                    std::io::stdout().flush().unwrap();
                },
                Ok(crate::lucid_schema_capnp::topic_response::Which::Delete(_deletes)) => {
                    write!(std::io::stdout(),"Topic Delete Response ------------\n").unwrap();
                    write!(std::io::stdout(),"Topic Name: {}\n", topic_response.get_topic_name().unwrap()).unwrap();
                    write!(std::io::stdout(),"Status: {}\n", topic_response.get_success()).unwrap();
                    std::io::stdout().flush().unwrap();
                },
                Err(_) => unimplemented!(),
            }


        },
        Ok(message_envelope::ProduceResponse(envelope_produce_response)) => {
            let produce_response = envelope_produce_response.expect("Unable to get produce request from envelope");
            write!(std::io::stdout(),"Produce Response ------------\n").unwrap();
            write!(std::io::stdout(),"Topic Name: {}\n", produce_response.get_topic_name().unwrap()).unwrap();
            write!(std::io::stdout(),"Status: {}\n", produce_response.get_success()).unwrap();
            write!(std::io::stdout(),"Last offset: {}\n", produce_response.get_offset()).unwrap();
            std::io::stdout().flush().unwrap();
        },
        Ok(message_envelope::ConsumeResponse(envelope_consume_response)) => {
            let consume_response = envelope_consume_response.unwrap();
            write!(std::io::stdout(),"Consume Response ------------\n").unwrap();
            write!(std::io::stdout(),"Topic Name: {}\n", consume_response.get_topic_name().unwrap()).unwrap();
            write!(std::io::stdout(),"Status: {}\n", consume_response.get_success()).unwrap();
            let messages = consume_response.get_messages().unwrap();
            let mut message_vec = Vec::new();
            //let second_vec = messages.iter().map(|x| format!("Key: {:?},Value: {:?}, Timestamp: {}", x.get_key().unwrap(), x.get_value().unwrap(), x.get_timestamp())).collect();
            for msg in messages {
                let message_string = format!("Key: {:?},Value: {:?}, Timestamp: {}", msg.get_key().unwrap(), msg.get_value().unwrap(), msg.get_timestamp());
                message_vec.push(message_string)
            }
            write!(std::io::stdout(),"Messages: {:?}\n", message_vec).unwrap();
            std::io::stdout().flush().unwrap();
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
            write!(std::io::stdout(),"Unable to parse cap n p message").unwrap();
            write!(std::io::stdout(),"\n").unwrap();
            std::io::stdout().flush().unwrap();
        }
    }
}