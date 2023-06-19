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

use crate::lucidmq_errors::BrokerError;

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
    pub fn new(directory: String) -> Result<Broker, BrokerError> {
        debug!("Creating new instance of lucidmq in {}", directory);
        //Try to load from file
        let lucidmq_file_path = Path::new(&directory).join("lucidmq.meta");
        let file_bytes = fs::read(lucidmq_file_path);
        match file_bytes {
            Ok(bytes) => {
                let decoded_lucidmq: Broker = bincode::deserialize(&bytes).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to deserialize lucidmq.meta file")
                })?;
                Ok(decoded_lucidmq)
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
                fs::create_dir_all(directory).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to create lucidmq directory")
                })?;
                Ok(lucidmq)
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
                    let data = self.handle_topic(capmessage).await.unwrap();
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ProduceRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_producer(capmessage).await.unwrap();
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ConsumeRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_consumer(capmessage).await.unwrap();
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

    async fn handle_topic(
        &mut self,
        topic_request_message: TypedReader<Builder<HeapAllocator>, topic_request::Owned>,
    ) -> Result<Vec<u8>, BrokerError> {
        let topic_request = topic_request_message.get().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to unpack topic request message")
        })?;
        let topic_name = topic_request.get_topic_name().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to unpack topic name from topic request")
        })?;
        match topic_request.which() {
            Ok(topic_request::Which::Create(_create_request)) => {
                Ok(self.handle_create_topic(topic_name))?
            }
            Ok(topic_request::Which::Delete(_delete_request)) => {
                Ok(self.handle_delete_topic(topic_name))?
            }
            Ok(topic_request::Which::Describe(_describe_request)) => {
                Ok(self.handle_describe_topic(topic_name))?
            }
            Err(_) => Err(BrokerError::new("Unknown topic request type")),
        }
    }

    fn handle_create_topic(&mut self, topic_name: &str) -> Result<Vec<u8>, BrokerError> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(_) => {
                warn!("topic already exisits");
                Ok(new_topic_response_create(topic_name, false))
            }
            None => {
                let topic = Topic::new(
                    topic_name.to_string(),
                    self.base_directory.clone(),
                    1000,
                    10000,
                );
                fs::create_dir_all(&topic.directory).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to create topic directory")
                })?;
                {
                    self.topics
                        .write()
                        .map_err(|e| {
                            error!("{}", e);
                            BrokerError::new("Unable to get write lock on topics")
                        })?
                        .push(Arc::new(RwLock::new(topic)));
                }
                self.flush();
                Ok(new_topic_response_create(topic_name, true))
            }
        }
    }

    fn handle_describe_topic(&mut self, topic_name: &str) -> Result<Vec<u8>, BrokerError> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(ind) => {
                let topics = self.topics.read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get read lock on topics")
                })?;
                let topic = topics.get(ind).unwrap().read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get read lock on single topic")
                })?;
                let cgs = topic.get_consumer_groups();
                let max_segment_size = topic.get_max_segment_size();
                info!("{}, {:?}, {}", topic_name, cgs, max_segment_size);
                Ok(new_topic_response_describe(
                    topic_name,
                    true,
                    topic.max_topic_size,
                    topic.max_segment_size,
                    cgs,
                ))
            }
            None => {
                warn!("topic does not exist");
                let dummy_vec = Vec::new();
                Ok(new_topic_response_describe(
                    topic_name, false, 0, 0, dummy_vec,
                ))
            }
        }
    }

    fn handle_delete_topic(&mut self, topic_name: &str) -> Result<Vec<u8>, BrokerError> {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(ind) => {
                // Get the topic directory
                let topics = self.topics.read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get read lock on topics")
                })?;
                let topic = topics.get(ind).unwrap().read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get read lock on single topic")
                })?;
                let topic_directory = &topic.directory.clone();
                // Clean up access since we dont need them anymore
                drop(topic);
                drop(topics);
                // Remove the topic from the topic vector
                self.topics
                    .write()
                    .map_err(|e| {
                        error!("{}", e);
                        BrokerError::new("Unable to get write lock on topics")
                    })?
                    .remove(ind);
                fs::remove_dir_all(topic_directory).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to delete diretory of topic")
                })?;
                self.flush();
                Ok(new_topic_response_delete(topic_name, true))
            }
            None => {
                warn!("topic does not exist");
                Ok(new_topic_response_delete(topic_name, false))
            }
        }
    }

    async fn handle_consumer(
        &mut self,
        consume_request: TypedReader<Builder<HeapAllocator>, consume_request::Owned>,
    ) -> Result<Vec<u8>, BrokerError> {
        info!("Handling consumer message");
        let consume_request_reader = consume_request.get().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get consume request from bytes")
        })?;
        let topic_name = consume_request_reader.get_topic_name().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get topic name from consume request")
        })?;
        let consumer_group = consume_request_reader.get_consumer_group().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get consumer group from consume request")
        })?;
        let timeout = consume_request_reader.get_timout();
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let broker = self.clone();
                let found_topic = &self.topics.read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get read lock on topic")
                })?[x];
                let consumer_group = found_topic
                    .write()
                    .map_err(|e| {
                        error!("{}", e);
                        BrokerError::new("Unable to get wrote lock on topic")
                    })?
                    .load_consumer_group(consumer_group);
                let mut consumer = Consumer::new(
                    found_topic.clone(),
                    consumer_group,
                    Box::new(move || broker.flush()),
                );
                let messages = consumer.poll(timeout).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to poll consumers commitlog")
                })?;
                let data = new_consume_response(topic_name, true, messages);
                Ok(data)
            }
            None => {
                warn!("topic does not exist");
                let message_data = Vec::new();
                let data = new_consume_response(topic_name, false, message_data);
                Ok(data)
            }
        }
    }

    async fn handle_producer(
        &mut self,
        produce_request: TypedReader<Builder<HeapAllocator>, produce_request::Owned>,
    ) -> Result<Vec<u8>, BrokerError> {
        info!("Handling producer message");
        let produce_request_reader = produce_request.get().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get produce request reader")
        })?;
        let topic_name = produce_request_reader.get_topic_name().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get topic name from produce request")
        })?;
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let found_topic = &self.topics.read().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get reader for topics")
                })?[x];
                let mut producer = Producer::new(found_topic.clone());
                // Parse out cap n proto produce messages and submit them to the commitlog
                let cap_msgs = produce_request_reader.get_messages().map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to get produce request messages")
                })?;
                let mut last_offset = 0;
                for msg in cap_msgs {
                    let mut builder_message = Builder::new_default();
                    builder_message.set_root(msg).map_err(|e| {
                        error!("{}", e);
                        BrokerError::new("Unable to set root for our produce request builder")
                    })?;
                    let bytes = serialize::write_message_to_words(&builder_message);
                    last_offset = producer.produce_bytes(&bytes).map_err(|e| {
                        error!("{}", e);
                        BrokerError::new("Unable to produce message to commitlog")
                    })?;
                }
                Ok(new_produce_response(topic_name, last_offset.into(), true))
            }
            None => Ok(new_produce_response(topic_name, 0, false)),
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
