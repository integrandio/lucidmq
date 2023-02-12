use std::{sync::{Arc, RwLock}};
use crate::{topic::Topic, consumer::Consumer, RecieverType, types::{Command, Payload}, types::SenderType};
use std::fs::{self, OpenOptions};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use log::{debug, info, warn, error};


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
                Command::Produce (payload) => {
                    let data = "my_payload".as_bytes().to_vec();
                    let produce_response = self.handle_producer("topic1", data);
                    let response_payload = Payload {
                        conn_id: payload.conn_id,
                        message: produce_response,
                        data: payload.data
                    };
                    Command::Response(response_payload)
                },
                Command::Consume (payload) => {
                    let consume_response = self.handle_consumer("topic1");
                    let response_payload = Payload {
                        conn_id: payload.conn_id,
                        message: consume_response,
                        data: payload.data
                    };
                    Command::Response(response_payload)
                }
                Command::Topic (payload) => {
                    let data = Vec::new();
                    let topic_response = self.new_topic("topic1");
                    let response_payload = Payload {
                        conn_id: payload.conn_id,
                        message: topic_response,
                        data: data
                    };
                    Command::Response(response_payload)
                }
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

    fn handle_consumer(&mut self, topic_name: &str) -> String{
        info!("Handling consumer message");
        let found_index = self.check_topics(topic_name);
        match found_index {
            Some(x) => {
                let broker = self.clone();
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                let consumer_group = found_topic.write().expect("unable to get writer").load_consumer_group("cg1");
                let mut consumer= Consumer::new(found_topic.clone(),
                     consumer_group,
                     Box::new(move || broker.flush()));
                let messages = consumer.poll(1000);
                info!("{}", messages.len());
                for message in messages {
                    info!("------------------------------------------");
                    info!("{}", std::str::from_utf8(&message.key).unwrap());
                    info!("{}", std::str::from_utf8(&message.value).unwrap());
                    info!("{:?}", &message.timestamp);
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
                found_topic.write().expect("unable to write to topic").commitlog.append(&data);
            },
            None => {
                warn!("topic does not exist");
            }
        }
        return "consumed".to_string();
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
                //let td = topic.directory.clone();
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
        let encoded_data: Vec<u8> =
            bincode::serialize(&self).expect("Unable to encode lucidmq metadata");
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(lucidmq_file_path)
            .expect("Unable to create and open file");

        file.write_all(&encoded_data)
            .expect("Unable to write to file");
    }
}