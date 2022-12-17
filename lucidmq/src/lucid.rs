use crate::consumer::Consumer;
use crate::producer::Producer;

use log::{debug};
use nolan::Commitlog;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::str;
use std::sync::atomic::Ordering;
use std::sync::{RwLock, Mutex};
use std::{sync::atomic::AtomicU32, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerGroup {
    name: String,
    pub offset: AtomicU32,
}

impl ConsumerGroup {
    pub fn new(consumer_group_name: String) -> ConsumerGroup {
        ConsumerGroup {
            name: consumer_group_name,
            offset: 0.into(),
        }
    }

    pub fn new_cg(consumer_group_name: String, offset_in: AtomicU32) -> ConsumerGroup {
        ConsumerGroup {
            name: consumer_group_name,
            offset: offset_in,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "DeserTopic")]
pub struct Topic {
    pub name: String,
    pub directory: String,
    consumer_groups: Vec<Arc<ConsumerGroup>>,
    #[serde(skip_serializing)]
    pub commitlog: Commitlog,
}

#[derive(Deserialize)]
struct DeserTopic {
    name: String,
    directory: String,
    consumer_groups: Vec<Arc<ConsumerGroup>>,
}

impl From<DeserTopic> for Topic {
    fn from(tmp: DeserTopic) -> Self {
        let commitlog = nolan::Commitlog::new(tmp.directory, 1000, 100);
        Self {
            name: tmp.name,
            directory: tmp.directory,
            consumer_groups: tmp.consumer_groups,
            commitlog: commitlog
        }
    }
}

impl Topic {
    pub fn new(topic_name: String, base_directory: String) -> Topic {
        debug!("Creating a new topic {}", topic_name);
        let path = Path::new(&base_directory);
        // Generate a random directory name
        let directory_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        let new_path = &path.join(directory_name);
        let new_consumer_groups = Vec::new();
        let new_commitlog = nolan::Commitlog::new(new_path
            .to_str()
            .expect("unable to convert to string")
            .to_string(), 1000, 100);
        Topic {
            name: topic_name,
            directory: new_path
                .to_str()
                .expect("unable to convert to string")
                .to_string(),
            consumer_groups: new_consumer_groups,
            commitlog: new_commitlog
        }
    }

    fn load_consumer_group(&mut self, consumer_group_name: String) -> Arc<ConsumerGroup> {
        for group in &self.consumer_groups {
            if group.name == consumer_group_name {
                return group.clone();
            }
        }
        let new_gc = Arc::new(ConsumerGroup::new(consumer_group_name));
        self.consumer_groups.push(new_gc.clone());
        new_gc
    }

    fn new_topic_from_ref(topic_ref: &Topic) -> Topic {
        let mut new_consumer_groups = Vec::new();
        for cg in &topic_ref.consumer_groups {
            new_consumer_groups.push(cg.clone());
        }
        let new_commitlog = nolan::Commitlog::new(topic_ref.directory.clone(), 1000, 100);
        Topic {
            name: topic_ref.name.clone(),
            directory: topic_ref.directory.clone(),
            consumer_groups: new_consumer_groups,
            commitlog: new_commitlog
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LucidMQ {
    pub base_directory: String,
    pub max_segment_bytes: u64,
    pub max_topic_size: u64,
    topics: Arc<RwLock<Vec<Arc<Mutex<Topic>>>>>,
}

impl LucidMQ {
    pub fn new(
        directory: String,
        max_segment_size_bytes: u64,
        max_topic_size_bytes: u64,
    ) -> LucidMQ {
        debug!("Creating new instance of lucidmq in {}", directory);
        //Try to load from file
        let lucidmq_file_path = Path::new(&directory).join("lucidmq.meta");
        let file_bytes = fs::read(lucidmq_file_path);
        match file_bytes {
            Ok(bytes) => {
                let decoded_lucidmq: LucidMQ =
                    bincode::deserialize(&bytes).expect("Unable to deserialize message");
                decoded_lucidmq
            }
            Err(_err) => {
                debug!("Lucid meta data file does not exist in directory {} creating a new file", directory);
                let lucidmq_vec = Vec::new();
                let lucidmq = LucidMQ {
                    base_directory: directory.clone(),
                    topics: Arc::new(RwLock::new(lucidmq_vec)),
                    max_segment_bytes: max_segment_size_bytes,
                    max_topic_size: max_topic_size_bytes,
                };
                fs::create_dir_all(directory).expect("Unable to create directory");
                lucidmq
            }
        }
    }
    
    pub fn new_topic(&mut self, topic_name: String) -> String {
        let topic = Topic::new(topic_name, self.base_directory.clone());
        fs::create_dir_all(&topic.directory).expect("Unable to create directory");
        let td = topic.directory.clone();
        {
            self.topics.write().unwrap().push(Arc::new(Mutex::new(topic)));
        }
        self.flush();
        td
    }

    pub fn new_producer(&mut self, topic: String) -> Producer {
        let found_index = self.check_topics(&topic);
        if found_index >= 0 {
            let usize_index: usize = found_index.try_into().expect("unable to convert");
            let found_topic = self.topics.read().unwrap()[usize_index];
            //let producer_topic = Arc::new(Mutex::new(found_topic));
            Producer::new(found_topic)
        } else {
            let new_topic = Topic::new(topic, self.base_directory.clone());
            let producer_topic = Arc::new(Mutex::new(new_topic));
            let producer = Producer::new(producer_topic);
            {
                self.topics.write().unwrap().push(producer_topic);
            }
            self.flush();
            producer
        }
    }

    pub fn new_consumer(&mut self, topic: String, consumer_group_name: String) -> Consumer {
        let lucidmq = self.clone();
        let found_index = self.check_topics(&topic);
        if found_index >= 0 {
            let usize_index: usize = found_index.try_into().expect("unable to convert");
            let found_topic = &self.topics.read().unwrap()[usize_index];
            //let consumer_topic = found_topic));
            let consumer_cg = found_topic.lock().unwrap().load_consumer_group(consumer_group_name.clone());
            Consumer::new(
                found_topic.to_owned(),
                consumer_cg,
                Box::new(move || lucidmq.sync(consumer_group_name.clone())),
            )
        } else {
            let user_cg = Arc::new(ConsumerGroup::new(consumer_group_name.clone()));
            let mut new_topic = Topic::new(topic, self.base_directory.clone());
            new_topic.consumer_groups.push(user_cg.clone());
            //let consumer_topic = new_topic));
            {
                self.topics.write().unwrap().push(Arc::new(Mutex::new(new_topic)));
            }
            
            Consumer::new(
                Arc::new(Mutex::new(new_topic)),
                user_cg,
                Box::new(move || lucidmq.sync(consumer_group_name.clone())),
            )
        }
    }

    fn check_topics(&mut self, topic_to_find: &String) -> i8 {
        if self.topics.read().unwrap().is_empty() {
            return -1;
        }
        let indexed_value = &self
            .topics
            .read()
            .unwrap()
            .iter()
            .position(|topic| topic.lock().unwrap().name == *topic_to_find);
        match indexed_value {
            None => -1,
            Some(index) => i8::try_from(*index).unwrap(),
        }
    }

    fn sync(&self, consumer_group_in_use: String) {
        let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
        let file_bytes = fs::read(lucidmq_file_path);
        match file_bytes {
            Ok(bytes) => {
                let decoded_lucidmq: LucidMQ =
                    bincode::deserialize(&bytes).expect("Unable to deserialize message");
                for topic in decoded_lucidmq.topics.read().unwrap().iter() {
                    let indexed_value = self
                        .topics
                        .read()
                        .unwrap()
                        .iter()
                        .position(|self_topic| self_topic.lock().unwrap().name == topic.lock().unwrap().name);
                    match indexed_value {
                        None => {
                            let topic_to_add = Topic::new_topic_from_ref(&topic.lock().unwrap());
                            self.topics.write().unwrap().push(Arc::new(Mutex::new(topic_to_add)));
                        }
                        Some(index) => {
                            let found_topic = &mut self.topics.write().unwrap()[index];
                            for cg in topic.lock().unwrap().consumer_groups.iter() {
                                let all_cgs = &found_topic.lock().unwrap().consumer_groups;
                                let found_cg = all_cgs.iter()
                                    .find(|self_cg| self_cg.name == cg.name);
                                match found_cg {
                                    None => {
                                        found_topic.lock().unwrap().consumer_groups.push(cg.clone());
                                    }
                                    Some(consumer_group) => {
                                        if consumer_group.name == consumer_group_in_use {
                                            continue;
                                        }
                                        let mut self_current_offset =
                                            consumer_group.offset.load(Ordering::SeqCst);
                                        let saved_current_offset = cg.offset.load(Ordering::SeqCst);
                                        //info!("Consumer Group updating {} current: {:?} new: {:?}", consumer_group.name, self_current_offset, saved_current_offset);
                                        loop {
                                            let res = consumer_group.offset.compare_exchange(
                                                self_current_offset,
                                                saved_current_offset,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst,
                                            );
                                            match res {
                                                Ok(_placeholder) => {
                                                    break;
                                                }
                                                Err(value) => {
                                                    //warn!("Unable to update consumer group offset {:?}", res);
                                                    self_current_offset = value;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
        self.flush()
    }

    fn flush(&self) {
        let lucidmq_file_path = Path::new(&self.base_directory).join("lucidmq.meta");
        debug!("Saving lucidmq state to file {}", lucidmq_file_path.to_string_lossy());
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
