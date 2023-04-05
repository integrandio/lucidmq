use crate::cap_n_proto_helper::{
    new_consume_response, new_produce_response, new_topic_response_create,
    new_topic_response_delete, new_topic_response_describe,
};
use crate::lucid_schema_capnp::{consume_request, produce_request, topic_request};
use crate::{
    consumer::Consumer, producer::Producer, topic::Topic, types::Command, types::SenderType,
    RecieverType,
};
use capnp::{
    message::{Builder, HeapAllocator, TypedReader},
    serialize,
};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "DeserBroker")]
pub struct Broker {
    pub base_directory: String,
    topics: Arc<RwLock<Vec<Arc<RwLock<Topic>>>>>,
}

#[derive(Deserialize)]
struct DeserBroker {
    pub base_directory: String,
    topics: Arc<RwLock<Vec<Arc<RwLock<Topic>>>>>,
}

impl From<DeserBroker> for Broker {
    fn from(tmp: DeserBroker) -> Self {
        Self {
            base_directory: tmp.base_directory,
            topics: tmp.topics,
        }
    }
}

impl Broker {
    pub fn new(directory: String) -> Broker {
        debug!("Creating new instance of lucidmq in {}", directory);
        //Try to load from file
        let lucidmq_file_path = Path::new(&directory).join("lucidmq.meta");
        let file_bytes = fs::read(lucidmq_file_path);
        match file_bytes {
            Ok(bytes) => {
                let decoded_lucidmq: Broker =
                    bincode::deserialize(&bytes).expect("Unable to deserialize message");
                decoded_lucidmq
            }
            Err(_err) => {
                info!(
                    "Lucid meta data file does not exist in directory {} creating a new file",
                    directory
                );
                let lucidmq_vec = Vec::new();
                let lucidmq = Broker {
                    base_directory: directory.clone(),
                    topics: Arc::new(RwLock::new(lucidmq_vec)),
                };
                fs::create_dir_all(directory).expect("Unable to create directory");
                lucidmq
            }
        }
    }

    pub async fn run(mut self, mut reciever: RecieverType, sender: SenderType) {
        info!("Broker is running");
        while let Some(command) = reciever.recv().await {
            info!("message came through {:?}", command);
            let response_command = match command {
                Command::TopicRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_topic(capmessage);
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ProduceRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_producer(capmessage);
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ConsumeRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_consumer(capmessage);
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                _ => {
                    warn!("Unable to parse command");
                    Command::Invalid {
                        message: "invalid".to_string(),
                    }
                }
            };
            let res = sender.send(response_command).await;
            match res {
                Err(e) => {
                    error!("{}", e)
                }
                Ok(_) => {}
            }
        }
    }

    fn handle_topic(
        &mut self,
        topic_request_message: TypedReader<Builder<HeapAllocator>, topic_request::Owned>,
    ) -> Vec<u8> {
        let topic_request = topic_request_message.get().unwrap();
        let topic_name = topic_request.get_topic_name().unwrap();
        match topic_request.which() {
            Ok(topic_request::Which::Create(_create_request)) => {
                self.handle_create_topic(topic_name)
            }
            Ok(topic_request::Which::Delete(_delete_request)) => {
                self.handle_delete_topic(topic_name)
            }
            Ok(topic_request::Which::Describe(_describe_request)) => {
                self.handle_describe_topic(topic_name)
            }
            Err(_) => {
                unimplemented!()
            }
        }
    }

    fn handle_create_topic(&mut self, topic_name: &str) -> Vec<u8> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(_) => {
                warn!("topic already exisits");
                new_topic_response_create(topic_name, false)
            }
            None => {
                let topic = Topic::new(
                    topic_name.to_string(),
                    self.base_directory.clone(),
                    1000,
                    10000,
                );
                fs::create_dir_all(&topic.directory).expect("Unable to create directory");
                {
                    self.topics
                        .write()
                        .expect("unable to get write lock")
                        .push(Arc::new(RwLock::new(topic)));
                }
                self.flush();
                new_topic_response_create(topic_name, true)
            }
        }
    }

    fn handle_describe_topic(&mut self, topic_name: &str) -> Vec<u8> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(ind) => {
                let topics = self.topics.read().unwrap();
                let topic = topics.get(ind).unwrap().read().unwrap();
                let cgs = topic.get_consumer_groups();
                let max_segment_size = topic.get_max_segment_size();
                info!("{}, {:?}, {}", topic_name, cgs, max_segment_size);
                new_topic_response_describe(topic_name, true, 0, max_segment_size, cgs)
            }
            None => {
                warn!("topic does not exist");
                let dummy_vec = Vec::new();
                new_topic_response_describe(topic_name, false, 0, 0, dummy_vec)
            }
        }
    }

    fn handle_delete_topic(&mut self, topic_name: &str) -> Vec<u8> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(ind) => {
                // Get the topic directory
                let topics = self.topics.read().unwrap();
                let topic = topics.get(ind).unwrap().read().unwrap();
                let topic_directory = &topic.directory.clone();
                // Clean up access since we dont need them anymore
                drop(topic);
                drop(topics);
                // Remove the topic from the topic vector
                self.topics
                    .write()
                    .expect("unable to get topics write lock")
                    .remove(ind);
                fs::remove_dir_all(topic_directory).expect("Unable to delete topic directory");
                self.flush();
                new_topic_response_delete(topic_name, true)
            }
            None => {
                warn!("topic does not exist");
                new_topic_response_delete(topic_name, false)
            }
        }
    }

    fn handle_consumer(
        &mut self,
        consume_request: TypedReader<Builder<HeapAllocator>, consume_request::Owned>,
    ) -> Vec<u8> {
        info!("Handling consumer message");
        let consume_request_reader = consume_request.get().unwrap();
        let topic_name = consume_request_reader.get_topic_name().unwrap();
        let consumer_group = consume_request_reader.get_consumer_group().unwrap();
        let timeout = consume_request_reader.get_timout();
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let broker = self.clone();
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                let consumer_group = found_topic
                    .write()
                    .expect("unable to get writer")
                    .load_consumer_group(consumer_group);
                let mut consumer = Consumer::new(
                    found_topic.clone(),
                    consumer_group,
                    Box::new(move || broker.flush()),
                );
                let messages = consumer.poll(timeout);
                let data = new_consume_response(topic_name, true, messages);
                return data;
            }
            None => {
                warn!("topic does not exist");
                let message_data = Vec::new();
                let data = new_consume_response(topic_name, false, message_data);
                return data;
            }
        }
    }

    fn handle_producer(
        &mut self,
        produce_request: TypedReader<Builder<HeapAllocator>, produce_request::Owned>,
    ) -> Vec<u8> {
        info!("Handling producer message");
        let req = produce_request.get().unwrap();
        let topic_name = req.get_topic_name().unwrap();
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                let mut producer = Producer::new(found_topic.clone());
                // Parse out cap n proto produce messages and submit them to the commitlog
                let cap_msgs = req.get_messages().unwrap();
                let mut last_offset = 0;
                for msg in cap_msgs {
                    let mut builder_message = Builder::new_default();
                    builder_message.set_root(msg).unwrap();
                    let bytes = serialize::write_message_to_words(&builder_message);
                    last_offset = producer.produce_bytes(&bytes);
                }
                return new_produce_response(topic_name, last_offset.into(), true);
            }
            None => {
                return new_produce_response(topic_name, 0, false);
            }
        }
    }

    fn check_topics(&mut self, topic_to_find: &str) -> Option<usize> {
        if self
            .topics
            .read()
            .expect("unable to get read lock")
            .is_empty()
        {
            return None;
        }
        let indexed_value = &self
            .topics
            .read()
            .expect("unable to get read lock")
            .iter()
            .position(|topic| {
                topic
                    .read()
                    .expect("Unable to read topic from read write lock")
                    .name
                    == *topic_to_find
            });
        match indexed_value {
            None => None,
            Some(index) => Some(*index),
        }
    }

    fn flush(&self) {
        let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
        info!(
            "Saving lucidmq state to file {}",
            lucidmq_file_path.to_string_lossy()
        );
        // TODO: error handle this
        let encoded_data: Vec<u8> =
            bincode::serialize(&self).expect("Unable to encode lucidmq metadata");
        // TODO: error handle this
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(lucidmq_file_path)
            .expect("Unable to create and open file");
        // TODO: error handle this
        file.write_all(&encoded_data)
            .expect("Unable to write to file");
    }
}
