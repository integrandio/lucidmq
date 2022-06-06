use nolan::commitlog::Commitlog;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::str;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Topic {
    name: String,
    directory: String
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
        let topic = Topic {
            name: topic_name,
            directory: new_path
                .to_str()
                .expect("unable to convert to string")
                .to_string(),
        };
        return topic;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

    pub fn new_consumer(&mut self, topic: String) -> Consumer {
        let found_index = self.check_topics(&topic);
        if found_index >= 0 {
            let usize_index: usize = found_index.try_into().expect("unable to convert");
            let found_topic = &mut self.topics[usize_index];
            let consumer = Consumer::new(found_topic.directory.clone(), found_topic.name.clone());

            return consumer;
        } else {
            let new_topic = Topic::new(topic, self.base_directory.clone());
            let consumer = Consumer::new(new_topic.directory.clone(), new_topic.name.clone());
            self.topics.push(new_topic);
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

    fn save(&self) {
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

pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_offset: usize,
}

impl Consumer {
    pub fn new(directory: String, topic: String) -> Consumer {
        let mut cl = Commitlog::new(directory.clone());
        let offset = &cl.get_oldest_offset();
        let consumer = Consumer {
            topic: topic,
            commitlog: cl,
            consumer_offset: *offset,
        };

        return consumer;
    }

    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {
        //Let's check if there are any new segments added.
        self.commitlog.reload_segments();

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            match self.commitlog.read(self.consumer_offset) {
                Ok(buffer) => {
                    let thing = Message::deserialize_message(&buffer);
                    records.push(thing);
                    self.consumer_offset = self.consumer_offset + 1;
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        //println!("Unable to get value for {}", self.consumer_offset);
                        self.commitlog.reload_segments();
                        thread::sleep(ten_millis);
                        elapsed_duration = start_time.elapsed();
                    } else {
                        panic!("Unexpected error found")
                    }
                }
            };
        }
        return records;
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }

    pub fn get_offset(&self) -> usize {
        return self.consumer_offset;
    }
}

pub struct Producer {
    topic: String,
    commitlog: Mutex<Commitlog>,
}

impl Producer {
    pub fn new(directory: String, topic: String) -> Producer {
        let cl = Commitlog::new(directory.clone());
        let consumer = Producer {
            topic: topic,
            commitlog: Mutex::new(cl),
        };

        return consumer;
    }

    pub fn produce(&mut self, message: &[u8]) {
        let mut cl = self.commitlog.lock().expect("lock has been poisoned...");
        cl.append(message);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(key: &[u8], value: &[u8], timestamp: Option<i64>) -> Message {
        let message_timestamp: i64;
        match timestamp {
            Some(timestamp) => {
                message_timestamp = timestamp;
            }
            None => {
                message_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;
            }
        }
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        let message = Message {
            key: key_vec,
            value: value_vec,
            timestamp: message_timestamp,
        };
        return message;
    }

    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        return encoded_message;
    }

    pub fn deserialize_message(message_bytes: &[u8]) -> Message {
        let decoded_message: Message =
            bincode::deserialize(message_bytes).expect("Unable to deserialize message");
        return decoded_message;
    }
}
