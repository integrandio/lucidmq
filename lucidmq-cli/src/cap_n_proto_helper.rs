use capnp::{serialize, message::ReaderOptions};

use crate::lucid_schema_capnp::{message_envelope};

pub fn parse_response(data: Vec<u8>) -> String {
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
                    let mut s = String::new();
                    s.push_str("Topic Create Response ------------\n");
                    let topic_name = format!("Topic Name: {}\n", topic_response.get_topic_name().unwrap());
                    s.push_str(&topic_name);
                    let status = format!("Status: {}\n", topic_response.get_success());
                    s.push_str(&status);
                    return s;
                },
                Ok(crate::lucid_schema_capnp::topic_response::Which::Describe(describe)) => {
                    let mut s = String::new();
                    s.push_str("Topic Describe Response ------------\n");
                    let topic_name = format!("Topic Name: {}\n", topic_response.get_topic_name().unwrap());
                    s.push_str(&topic_name);
                    let status = format!("Status: {}\n", topic_response.get_success());
                    s.push_str(&status);
                    let cgs = describe.get_consumer_groups().unwrap();
                    let mut cgs_vec = Vec::new();
                    for msg in cgs {
                        cgs_vec.push(msg.unwrap().to_string())
                    }
                    let topic_details = format!("Topic max retention bytes: {}, max segments bytes: {}, consumer groups: {:?}\n", describe.get_max_retention_bytes(), describe.get_max_segment_bytes(), cgs_vec);
                    s.push_str(&topic_details);
                    return s;
                },
                Ok(crate::lucid_schema_capnp::topic_response::Which::Delete(_deletes)) => {
                    let mut s = String::new();
                    let topic_name = format!("Topic Name: {}\n", topic_response.get_topic_name().unwrap());
                    let status = format!("Status: {}\n", topic_response.get_success());
                    s.push_str("Topic Delete Response ------------\n");
                    s.push_str(&topic_name);
                    s.push_str(&status);
                    return s;
                },
                Err(_) => unimplemented!(),
            }
        },
        Ok(message_envelope::ProduceResponse(envelope_produce_response)) => {
            let produce_response = envelope_produce_response.expect("Unable to get produce request from envelope");
            let topic_name = format!("Topic Name: {}\n", produce_response.get_topic_name().unwrap());
            let status = format!("Status: {}\n", produce_response.get_success());
            let offset = format!("Last offset: {}\n", produce_response.get_offset());
            let mut s = String::new();
            s.push_str("Produce Response ------------\n");
            s.push_str(&topic_name);
            s.push_str(&status);
            s.push_str(&offset);
            return s;
        },
        Ok(message_envelope::ConsumeResponse(envelope_consume_response)) => {
            let consume_response = envelope_consume_response.unwrap();
            let topic_name = format!("Topic Name: {}\n", consume_response.get_topic_name().unwrap());
            let status = format!("Status: {}\n", consume_response.get_success());
            let mut s = String::new();

            let messages = consume_response.get_messages().unwrap();
            let mut message_vec = Vec::new();
            //let second_vec = messages.iter().map(|x| format!("Key: {:?},Value: {:?}, Timestamp: {}", x.get_key().unwrap(), x.get_value().unwrap(), x.get_timestamp())).collect();
            for msg in messages {
                let message_string = format!("Key: {:?},Value: {:?}, Timestamp: {}", msg.get_key().unwrap(), msg.get_value().unwrap(), msg.get_timestamp());
                message_vec.push(message_string)
            }
            let messages = format!("Messages: {:?}\n", message_vec);
            s.push_str("Consume Response ------------\n");
            s.push_str(&topic_name);
            s.push_str(&status);
            s.push_str(&messages);
            return s;
        },
        Ok(message_envelope::TopicRequest(_envelope_topic_request)) => {
            return "Invalid request type".to_string();
        },
        Ok(message_envelope::ConsumeRequest(_envelope_produce_request)) => {
            return "Invalid request type".to_string();
        },
        Ok(message_envelope::ProduceRequest(_envelope_consume_request)) => {
            return "Invalid request type".to_string();
        },
        Err(::capnp::NotInSchema(_)) => {
            return "Unable to parse cap n p message\n".to_string();
        }
    }
}