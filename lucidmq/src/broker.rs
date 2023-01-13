use std::{sync::{Arc, RwLock}};
use crate::{topic::Topic, message::Message, consumer::Consumer, RecieverType, Command, SenderType};
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
            let thing: Command;
            info!("message came through {:?}", command);
            match command {
                Command::Produce { key: _ } => {
                    let thing0= self.handle_producer("topic1");
                    thing = Command::Response { key: thing0 }
                },
                Command::Consume { key: _ } => {
                    let thing0 = self.handle_consumer("topic1");
                    thing = Command::Response { key: thing0 }
                }
                Command::Topic { key: _ } => {
                    let thing0 = self.new_topic("topic1");
                    thing = Command::Response { key: thing0 }
                }
                _=> {
                    warn!("Unable to parse command");
                    thing = Command::Invalid { key: "invalid".to_string() }
                } 
            }
            let res = sender.send(thing).await;
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
        return "produced".to_string();
    }

    fn handle_producer(&mut self, topic_name: &str) -> String{
        info!("Handling producer message");
        let found_index = self.check_topics(topic_name);
        let mut message = Message::new("key".as_bytes(), "value".as_bytes(), None);
        
        match found_index {
            Some(x) => {
                let found_topic = &self.topics.read().expect("unable to get read lock")[x];
                found_topic.write().expect("unable to write to topic").commitlog.append(&message.serialize_message());
            },
            None => {
                warn!("topic does not exist");
            }
        }
        return "consumed".to_string();
    }
    
    fn new_topic(&mut self, topic_name: &str) -> String{
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

    // fn sync(&self, consumer_group_in_use: &str) {
    //     let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
    //     let file_bytes = fs::read(lucidmq_file_path);
    //     match file_bytes {
    //         Ok(bytes) => {
    //             let decoded_lucidmq: Broker =
    //                 bincode::deserialize(&bytes).expect("Unable to deserialize message");
    //             for topic in decoded_lucidmq.topics.read().unwrap().iter() {
    //                 let indexed_value = self
    //                     .topics
    //                     .read()
    //                     .unwrap()
    //                     .iter()
    //                     .position(|self_topic| self_topic.read().unwrap().name == topic.read().unwrap().name);
    //                 match indexed_value {
    //                     None => {
    //                         let topic_to_add = Topic::new_topic_from_ref(&topic.read().unwrap());
    //                         self.topics.write().unwrap().push(Arc::new(RwLock::new(topic_to_add)));
    //                     }
    //                     Some(index) => {
    //                         let found_topic = &mut self.topics.write().unwrap()[index];
    //                         for cg in topic.read().unwrap().consumer_groups.iter() {
    //                             let all_cgs = &found_topic.read().unwrap().consumer_groups;
    //                             let found_cg = all_cgs.iter()
    //                                 .find(|self_cg| self_cg.name == cg.name);
    //                             match found_cg {
    //                                 None => {
    //                                     found_topic.write().unwrap().consumer_groups.push(cg.clone());
    //                                 }
    //                                 Some(consumer_group) => {
    //                                     if consumer_group.name == consumer_group_in_use {
    //                                         continue;
    //                                     }
    //                                     let mut self_current_offset =
    //                                         consumer_group.offset.load(Ordering::SeqCst);
    //                                     let saved_current_offset = cg.offset.load(Ordering::SeqCst);
    //                                     info!("Consumer Group updating {} current: {:?} new: {:?}", consumer_group.name, self_current_offset, saved_current_offset);
    //                                     loop {
    //                                         let res = consumer_group.offset.compare_exchange(
    //                                             self_current_offset,
    //                                             saved_current_offset,
    //                                             Ordering::SeqCst,
    //                                             Ordering::SeqCst,
    //                                         );
    //                                         match res {
    //                                             Ok(_placeholder) => {
    //                                                 break;
    //                                             }
    //                                             Err(value) => {
    //                                                 warn!("Unable to update consumer group offset {:?}", res);
    //                                                 self_current_offset = value;
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             panic!("{}", err)
    //         }
    //     }
    //     self.flush()
    // }

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