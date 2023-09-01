use crate::cap_n_proto_helper::{
    new_consume_response, new_produce_response, new_topic_response_create,
    new_topic_response_delete, new_topic_response_describe, new_topic_response_all, new_invalid_response
};
use crate::lucid_schema_capnp::{consume_request, produce_request, topic_request};
use crate::{
    consumer::Consumer, producer::Producer, topic::Topic, types::Command, types::SenderType,
    types::RecieverType, topic::SimpleTopic
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

    pub async fn run(mut self, mut reciever: RecieverType, sender: SenderType) -> Result<(), BrokerError> {
        info!("Broker is running");
        while let Some(command) = reciever.recv().await {
            info!("message came through {:?}", command);
            let response_command = match command {
                Command::TopicRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_topic(capmessage).await?;
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ProduceRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_producer(capmessage).await?;
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::ConsumeRequest {
                    conn_id,
                    capmessage,
                } => {
                    let data = self.handle_consumer(capmessage).await?;
                    Command::Response {
                        conn_id: conn_id,
                        capmessagedata: data,
                    }
                }
                Command::Invalid { conn_id, error_message,  capmessage_data:_} => {
                    let data = self.handle_invalid_message(&error_message).await?;
                    Command::Invalid {
                        conn_id: conn_id,
                        error_message: error_message,
                        capmessage_data: data
                    }
                }
                Command::Response { conn_id, capmessagedata:_ } => {
                    warn!("Response type unexected command");
                    let data = self.handle_invalid_message("Response message is invalid").await?;
                    Command::Invalid {
                        conn_id: conn_id,
                        error_message: "Response message is invalid".to_string(),
                        capmessage_data: data
                    }
                    
                },
            };
            let res = sender.send(response_command).await;
            match res {
                Err(e) => {
                    error!("{}", e);
                    return Err(BrokerError::new("Unable to send message"));
                }
                Ok(_) => {}
            }
        }
        Ok(())
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
            },
            Ok(topic_request::Which::All(_all_request)) => {
                Ok(self.handle_all_topic())?
            },
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
                    100000, //100kb
                    1000000, //1mb
                ).map_err(|err| {
                    error!("{}", err);
                    BrokerError::new("Unable to create topic directory")
                })?;
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

    fn handle_all_topic(&mut self) -> Result<Vec<u8>, BrokerError> {
        if self.topics.read().map_err(|e| {
            error!("{}", e);
            BrokerError::new("Unable to get read lock on topics")
        })?.is_empty(){
            return Err(BrokerError::new("Unable to get topic name from consume request"));
        }
        let mut simple_topics = Vec::new();
        for topic in self.topics.read().expect("unable to get topic lock").iter() {
            let topic_name = &topic.read().unwrap().name;
            let consumer_groups = topic.read().unwrap().get_consumer_groups();
            let st = SimpleTopic {
                topic_name: topic_name.to_string(),
                consumer_groups: consumer_groups
            };
            simple_topics.push(st)
        }
        let response_data = new_topic_response_all(true, simple_topics);
        Ok(response_data)
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
                ).map_err(|e| {
                    error!("{}", e);
                    BrokerError::new("Unable to create new consumer")
                })?;
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
                // Let's insert multiple messages here, instead of iterating through each one.
                // let mut builder_message = Builder::new_default();
                // builder_message.set_root(cap_msgs).unwrap();
                // serialize::write_message_to_words(&builder_message);
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
            None => {
                warn!("Topic {} does not exist", topic_name);
                Ok(new_produce_response(topic_name, 0, false))
            }
        }
    }

    async fn handle_invalid_message(&self, message_text: &str) -> Result<Vec<u8>, BrokerError> {
        let data = new_invalid_response(message_text);
        Ok(data)
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

#[cfg(test)]
mod broker_tests {
    use crate::broker::Broker;
    use tempdir::TempDir;
    use std::path::Path;

    #[test]
    fn test_new_broker() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let broker = Broker::new(String::from(tmp_dir_string)).expect("unable to create new broker");
        assert!(Path::new(&broker.base_directory).is_dir());
    }
    // Tests to write:
    // - happy path broker, directory and lucidmq meta are created
    // - handle run, send message of each kind, verify the response including invalid
    // - topic handler function, try each kind
    // - producer handler
    // - consumer handler
    // - check topics
    // - test flush

}