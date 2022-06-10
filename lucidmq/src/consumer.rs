use log::info;
use nolan::commitlog::Commitlog;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};
use crate::lucidmq::ConsumerGroup;
use crate::message::{Message};

pub struct Consumer {
    topic: String,
    commitlog: Commitlog,
    consumer_offset: usize,
    consumer_group: Arc<ConsumerGroup>
}

impl Consumer {
    pub fn new(directory: String, topic: String, consumer_group: Arc<ConsumerGroup>) -> Consumer {
        let mut cl = Commitlog::new(directory.clone());
        let offset = &cl.get_oldest_offset();
        let consumer = Consumer {
            topic: topic,
            commitlog: cl,
            consumer_offset: *offset,
            consumer_group: consumer_group
        };

        return consumer;
    }

    pub fn poll(&mut self, timeout: u64) -> Vec<Message> {
        //Let's check if there are any new segments added.
        self.commitlog.reload_segments();

        info!{"{:?}", self.consumer_group};

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Message> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        while timeout_duration > elapsed_duration {
            let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).unwrap();
            match self.commitlog.read(n) {
                Ok(buffer) => {
                    let thing = Message::deserialize_message(&buffer);
                    records.push(thing);
                    //self.consumer_offset = self.consumer_offset + 1;
                    self.update_consumer_group_offset();
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

    pub fn update_consumer_group_offset(&mut self) {
        self.consumer_group.offset.fetch_add(1, Ordering::SeqCst);
    }
}
