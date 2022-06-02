
use pyo3::{prelude::*};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;
use nolan::{commitlog::Commitlog};

#[pyclass]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message{
    #[pyo3(get, set)]
    key: Vec<u8>,
    #[pyo3(get, set)]
    value: Vec<u8>,
    #[pyo3(get, set)]
    timestamp: i64,
}

#[pymethods]
impl Message {
    // #[new]
    // pub fn py_new(key: &[u8], value: &[u8], timestamp: i64) -> PyResult<Message> {
    //     let key_vec = key.to_vec();
    //     let value_vec = value.to_vec();
    //     let message = Message {
    //         key: key_vec,
    //         value: value_vec,
    //         timestamp: timestamp,
    //     };
    //     return message;
    // }
    #[new]
    pub fn py_new_without_timestamp(key: &[u8], value: &[u8]) -> Message {
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let message = Message {
            key: key_vec,
            value: value_vec,
            timestamp: timestamp,
        };
        return message;
    }
    pub fn serialize_message(&mut self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::serialize(&self).expect("Unable to encode message");
        return encoded_message;
    }

    // pub fn deserialize_message(_py: Python, message_bytes: &PyBytes) -> Message {
    //     let decoded_message: Message = bincode::deserialize(message_bytes.as_bytes()).expect("Unable to deserialize message");
    //     return decoded_message;
    // }
}

//This should go above but it doesnt work lol
pub fn deserialize_message(message_bytes: &[u8]) -> Message {
    let decoded_message: Message = bincode::deserialize(message_bytes).expect("Unable to deserialize message");
    return decoded_message;
}

#[pyclass]
pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_offset: usize,
}

#[pymethods]
impl Consumer {
    #[new]
    pub fn new(topic: String) -> Consumer {
        let path = String::from("test");
        let cl = Commitlog::new(path);

        let consumer = Consumer {
            topic: topic,
            commitlog: cl,
            consumer_offset: 0,
        };

        return consumer;
    }

    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {
        self.commitlog.reload_segments();
        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> =  Vec::new();
        let start_time = Instant::now();

        // Should we try to load segments here?

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            match self.commitlog.read(self.consumer_offset) {
                Ok(buffer) => {
                    let thing = deserialize_message(&buffer);
                    records.push(thing);
                    self.consumer_offset = self.consumer_offset + 1;
                }
                Err(err) => {
                    // Should we try to load segments here?
                    
                    if err == "Offset does not exist in the commitlog" {
                        println!("Unable to get value for {}", self.consumer_offset);
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

    // pub fn consume(&mut self, offset: usize) {
    //     self.commitlog.read(offset);
    // }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}


#[pyclass]
pub struct Producer {
    topic: String,
    commitlog: Commitlog,
}

#[pymethods]
impl Producer {
    #[new]
    pub fn new(topic: String) -> Producer {
        let path = String::from("test");
        let cl = Commitlog::new(path);

        let consumer = Producer {
            topic: topic,
            commitlog: cl,
        };

        return consumer;
    }

    pub fn produce_message(&mut self, mut message: Message) {
        let vector_thing = message.serialize_message();
        let vec_point = &vector_thing;
        let c: &[u8] = &vec_point;
        self.commitlog.append(c);
    }

    pub fn produce(&mut self, message: &[u8]) {
        self.commitlog.append(message);
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn pylucidmq(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Message>()?;
    //m.add_function(wrap_pyfunction!(deserialize_message, m)?)?;
    m.add_class::<Producer>()?;
    m.add_class::<Consumer>()?;
    //m.add_function(deserialize_message);
    Ok(())
}