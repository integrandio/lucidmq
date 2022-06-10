use crate::consumer::Consumer;
use crate::producer::Producer;
use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::str;
use std::{sync::atomic::AtomicU32, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerGroup {
    name: String,
    pub offset: AtomicU32,
}

impl ConsumerGroup {
    pub fn new(consumer_group_name: String) -> ConsumerGroup {
        let consumer_group = ConsumerGroup {
            name: consumer_group_name,
            offset: 0.into(),
        };
        return consumer_group;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    name: String,
    directory: String,
    consumer_groups: Vec<Arc<ConsumerGroup>>,
}

impl Topic {
    pub fn new(topic_name: String, base_directory: String) -> Topic {
        let path = Path::new(&base_directory);
        // Generate a random directory name
        let directory_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        let new_path = &path.join(directory_name);
        let consumer_groups = Vec::new();
        let topic = Topic {
            name: topic_name,
            directory: new_path
                .to_str()
                .expect("unable to convert to string")
                .to_string(),
            consumer_groups: consumer_groups,
        };
        return topic;
    }

    pub fn load_consumer_group(&mut self, consumer_group_name: String) -> Arc<ConsumerGroup> {
        for group in &self.consumer_groups {
            if group.name == consumer_group_name {
                return group.clone();
            }
        }
        let new_gc = Arc::new(ConsumerGroup::new(consumer_group_name));
        self.consumer_groups.push(new_gc.clone());
        return new_gc;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LucidMQ {
    base_directory: String,
    topics: Vec<Topic>,
}

impl LucidMQ {
    pub fn new(directory: String) -> LucidMQ {
        //Try to load from file
        let lucidmq_file_path = Path::new(&directory).join("lucidmq.meta");
        let file_bytes = fs::read(lucidmq_file_path);
        match file_bytes {
            Ok(bytes) => {
                let decoded_lucidmq: LucidMQ =
                    bincode::deserialize(&bytes).expect("Unable to deserialize message");
                //info!("{:?}", decoded_lucidmq);
                return decoded_lucidmq;
            }
            Err(_err) => {
                let lucidmq_vec = Vec::new();
                let lucidmq = LucidMQ {
                    base_directory: directory.clone(),
                    topics: lucidmq_vec,
                };
                fs::create_dir_all(directory).expect("Unable to create directory");
                return lucidmq;
            }
        }
    }

    pub fn new_producer(&mut self, topic: String) -> Producer {
        let found_index = self.check_topics(&topic);
        if found_index >= 0 {
            let usize_index: usize = found_index.try_into().expect("unable to convert");
            let found_topic = &self.topics[usize_index];
            let producer = Producer::new(found_topic.directory.clone(), found_topic.name.clone());
            return producer;
        } else {
            let new_topic = Topic::new(topic, self.base_directory.clone());
            let producer = Producer::new(new_topic.directory.clone(), new_topic.name.clone());
            self.topics.push(new_topic);
            self.save();
            return producer;
        }
    }

    pub fn new_consumer(&mut self, topic: String, consumer_group_name: String) -> Consumer {
        let found_index = self.check_topics(&topic);
        if found_index >= 0 {
            let usize_index: usize = found_index.try_into().expect("unable to convert");
            let found_topic = &mut self.topics[usize_index];
            //info!("{:?}", found_topic);
            let consumer_cg = found_topic.load_consumer_group(consumer_group_name);
            let consumer = Consumer::new(
                found_topic.directory.clone(),
                found_topic.name.clone(),
                consumer_cg,
            );
            info!("Debugging here {:?}", self);
            self.save();
            return consumer;
        } else {
            let user_cg = Arc::new(ConsumerGroup::new(consumer_group_name));
            let mut new_topic = Topic::new(topic, self.base_directory.clone());
            new_topic.consumer_groups.push(user_cg.clone());
            let consumer =
                Consumer::new(new_topic.directory.clone(), new_topic.name.clone(), user_cg);
            self.topics.push(new_topic);
            info!("Debugging here {:?}", self);
            self.save();
            return consumer;
        }
    }

    fn check_topics(&mut self, topic_to_find: &String) -> i8 {
        if self.topics.is_empty() {
            return -1;
        }
        let mut i: i8 = 0;
        for topic in &self.topics {
            if topic.name == *topic_to_find {
                return i;
            }
            i += 1
        }
        return -1;
    }

    pub fn save(&self) {
        info!("Saving file...");
        let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
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
