use crate::lucidmq::ConsumerGroup;
use crate::message::Message;
use nolan::Commitlog;
use pyo3::prelude::*;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[pyclass(unsendable)]
pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_group: Arc<ConsumerGroup>,
    cb: Box<dyn Fn()>,
}

impl Consumer {
    /**
     * Initializes a new consumer
     */
    pub fn new(
        directory: String,
        topic_name: String,
        new_consumer_group: Arc<ConsumerGroup>,
        callback: Box<dyn Fn()>,
        max_segment_size_bytes: u64,
        max_commitlog_size_bytes: u64,
    ) -> Consumer {
        let cl = Commitlog::new(directory, max_segment_size_bytes, max_commitlog_size_bytes);
        let mut consumer = Consumer {
            topic: topic_name,
            commitlog: cl,
            consumer_group: new_consumer_group,
            cb: callback,
        };
        consumer.consumer_group_initialize();
        consumer
    }
}

#[pymethods]
impl Consumer {
    /**
     * Reads from the commitlog for a set amount of time and returns a vector is messages when complete.
     */
    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {
        //Let's check if there are any new segments added.
        self.commitlog.reload_segments();

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).unwrap();
            match self.commitlog.read(n) {
                Ok(buffer) => {
                    let message = Message::deserialize_message(&buffer);
                    records.push(message);
                    self.update_consumer_group_offset();
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        self.commitlog.reload_segments();
                        thread::sleep(ten_millis);
                        elapsed_duration = start_time.elapsed();
                    } else {
                        panic!("Unexpected error found")
                    }
                }
            };
        }
        if !records.is_empty() {
            self.save_info();
        }
        records
    }

    /**
     * Given a starting offset and a max_records to return, fetch will read all of the offsets and return the records until there is no more records
     * or the max records limit has been hit.
     */
    pub fn fetch(&mut self, starting_offset: usize, max_records: usize) -> Vec<Message> {
        self.commitlog.reload_segments();
        let mut offset = starting_offset;
        let mut records: Vec<Message> = Vec::new();
        while records.len() < max_records {
            match self.commitlog.read(offset) {
                Ok(buffer) => {
                    let message = Message::deserialize_message(&buffer);
                    records.push(message);
                    offset += 1;
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        break;
                    } else {
                        panic!("Unexpected error found")
                    }
                }
            };
        }
        records
    }

    /**
     * Returns the topic that the consumer is consuming from.
     */
    pub fn get_topic(&self) -> String {
        self.topic.clone()
    }
    
    pub fn get_oldest_offset(&mut self) -> usize{
        self.commitlog.get_oldest_offset()
    }

    pub fn get_latest_offset(&mut self) -> usize{
        self.commitlog.get_latest_offset()
    }

    /**
     * Verifies and fixes if the consumer group is set to something that is older than the oldest offset in the commitlog.
     * If it is older, it will set it to the oldest possible offset.
     */
    fn consumer_group_initialize(&mut self) {
        let oldest_offset = self.commitlog.get_oldest_offset();
        let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).unwrap();
        if n < oldest_offset {
            let new_consumer_group_offset: u32 = oldest_offset.try_into().unwrap();
            self.consumer_group
                .offset
                .store(new_consumer_group_offset, Ordering::SeqCst);
            self.save_info();
        }
    }

    /**
     * Updates the consumer_group offset counter by 1.
     */
    pub fn update_consumer_group_offset(&mut self) {
        self.consumer_group.offset.fetch_add(1, Ordering::SeqCst);
    }

    /**
     * save info calls a callback function which will sync and persist the state.
     */
    fn save_info(&self) {
        (self.cb)()
    }
}
