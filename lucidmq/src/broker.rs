use std::{sync::{Arc, RwLock}};
use crate::{topic::Topic, consumer::Consumer, RecieverType, types::{Command}, types::SenderType, producer::Producer};
use std::fs::{self, OpenOptions};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use log::{debug, info, warn, error};
use crate::cap_n_proto_helper::{new_topic_response, new_produce_response, new_consume_response};


#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "DeserBroker")]
pub struct Broker {
    pub base_directory: String,
    pub max_segment_bytes: u64,
    pub max_topic_size: u64,
    topics: Arc<RwLock<Vec<Arc<RwLock<Topic>>>>>
}

#[derive(Deserialize)]
struct DeserBroker {
    pub base_directory: String,
    pub max_segment_bytes: u64,
    pub max_topic_size: u64,
    topics: Arc<RwLock<Vec<Arc<RwLock<Topic>>>>>
}

impl From<DeserBroker> for Broker {
    fn from(tmp: DeserBroker) -> Self {
        Self {
            base_directory: tmp.base_directory,
            max_segment_bytes: tmp.max_segment_bytes,
            max_topic_size: tmp.max_topic_size,
            topics: tmp.topics
        }
    }
}

impl Broker {
    pub fn new(
        directory: String,
        max_segment_size_bytes: u64,
        max_topic_size_bytes: u64,
    ) -> Broker {
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
                info!("Lucid meta data file does not exist in directory {} creating a new file", directory);
                let lucidmq_vec = Vec::new();
                let lucidmq = Broker {
                    base_directory: directory.clone(),
                    topics: Arc::new(RwLock::new(lucidmq_vec)),
                    max_segment_bytes: max_segment_size_bytes,
                    max_topic_size: max_topic_size_bytes
                };
                fs::create_dir_all(directory).expect("Unable to create directory");
                lucidmq
            }
        }
    }

    pub async fn run(mut self, mut reciever: RecieverType, sender: SenderType) {
        while let Some(command) = reciever.recv().await {
            info!("message came through {:?}", command);
            let response_command = match command {
                Command::TopicRequest { conn_id, capmessage } => {
                    let cp_request = capmessage.get().unwrap();
                    let data = new_topic_response();
                    let _topic_response = self.new_topic(cp_request.get_topic_name().unwrap());
                    Command::Response { conn_id: conn_id, capmessagedata: data}
                }
                Command::ProduceRequest { conn_id, capmessage } => {
                    let cp_request = capmessage.get().unwrap();
                    let data = new_produce_response();
                    let _produce_response = self.handle_producer(cp_request.get_topic_name().unwrap(), "abcdefg".as_bytes().to_vec());
                    Command::Response { conn_id: conn_id, capmessagedata: data}
                },
                Command::ConsumeRequest { conn_id, capmessage } => {
                    let cp_request = capmessage.get().unwrap();
                    
                    let data = new_consume_response();
                    let _consume_response = self.handle_consumer(
                        cp_request.get_topic_name().unwrap(),
                         cp_request.get_consumer_group().unwrap()
                    );
                    Command::Response { conn_id: conn_id, capmessagedata: data}
                },
                _=> {
                    warn!("Unable to parse command");
                    Command::Invalid { message: "invalid".to_string() }
                } 
            };
            let res = sender.send(response_command).await;
            match res {
                Err(e) => {
                    error!("{}", e)
                }
                Ok(_) => {},
            }
        }
    }

    fn handle_consumer(&mut self, topic_name: &str, consumer_group: &str) -> String{
        info!("Handling consumer message");
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let broker = self.clone();
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                let consumer_group = found_topic.write().expect("unable to get writer").load_consumer_group(consumer_group);
                let mut consumer= Consumer::new(found_topic.clone(),
                     consumer_group,
                     Box::new(move || broker.flush()));
                let messages = consumer.poll(1000);
                info!("{}", messages.len());
                for message in messages {
                    info!("------------------------------------------");
                    info!("{:?}", message);
                }
            },
            None => {
                warn!("topic does not exist");
            }
        } 
        return "consumed".to_string();
    }

    fn handle_producer(&mut self, topic_name: &str, data: Vec<u8>) -> String{
        info!("Handling producer message");
        let found_index = self.check_topics(topic_name);

        match found_index {
            Some(x) => {
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                let mut producer = Producer::new(found_topic.clone());
                producer.produce_bytes(&data);
            },
            None => {
                warn!("topic does not exist");
            }
        }
        return "produced".to_string();
    }
    
    fn new_topic(&mut self, topic_name: &str) -> String {
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(_) => {
                warn!("topic already exisits")
            },
            None => {
                let topic = Topic::new(topic_name.to_string(), self.base_directory.clone());
                fs::create_dir_all(&topic.directory).expect("Unable to create directory");
                {
                    self.topics.write().expect("unable to get write lock").push(Arc::new(RwLock::new(topic)));
                }
                self.flush();
            }
        }
        return "topiced".to_string()
        
    }

    fn check_topics(&mut self, topic_to_find: &str) -> Option<usize> {
        if self.topics.read().expect("unable to get read lock").is_empty() {
            return None;
        }
        let indexed_value = &self
            .topics
            .read()
            .expect("unable to get read lock")
            .iter()
            .position(|topic| 
            topic.read().
            expect("Unable to read topic from read write lock").name== *topic_to_find);
        match indexed_value {
            None => None,
            Some(index) => Some(*index),
        }
    }

    fn flush(&self) {
        let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
        info!("Saving lucidmq state to file {}", lucidmq_file_path.to_string_lossy());
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