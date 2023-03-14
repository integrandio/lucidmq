use crate::topic::{Topic, ConsumerGroup};
use log::info;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

/// Consumer struct for directly interacting with the commitlog in a consuming fashion
pub struct Consumer {
    topic: Arc<RwLock<Topic>>,
    consumer_group: Arc<ConsumerGroup>,
    cb: Box<dyn Fn()>,
}

impl Consumer {
    /**
     * Initializes a new consumer
     */
    pub fn new(
        consumer_topic: Arc<RwLock<Topic>>,
        new_consumer_group: Arc<ConsumerGroup>,
        callback: Box<dyn Fn()>,
    ) -> Consumer {
        let mut consumer = Consumer {
            topic: consumer_topic,
            consumer_group: new_consumer_group,
            cb: callback,
        };
        consumer.consumer_group_initialize();
        consumer
    }

    /**
     * Reads from the commitlog for a set amount of time and returns a vector is messages when complete. 
     * The offset where the starting read takes place is based off of the consumer group offset
     */
    pub fn poll(&mut self, timeout: u64) -> Vec<Vec<u8>> {
        //Let's check if there are any new segments added.
        self.topic.write().expect("Unable to get lock on consumer topic").commitlog.reload_segments();
        info!("polling for messages");

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Vec<u8>> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        //let mut n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).expect("Unable to get offset");
        while timeout_duration > elapsed_duration {
            let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).expect("Unable to get offset");
            let mut topic = self.topic.write().expect("Unable to get lock on consumer topic");
            match topic.commitlog.read(n) {
                Ok(buffer) => {
                    records.push(buffer);
                    self.update_consumer_group_offset();
                }
                Err(err) => {
                    if err == "Offset does not exist in the commitlog" {
                        topic.commitlog.reload_segments();
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
    pub fn _fetch(&mut self, starting_offset: usize, max_records: usize) -> Vec<Vec<u8>> {
        let commitlog = &mut self.topic.write().expect("Unable to get topic from lock").commitlog;
        commitlog.reload_segments();
        let mut offset = starting_offset;
        let mut records: Vec<Vec<u8>> = Vec::new();
        while records.len() < max_records {
            match commitlog.read(offset) {
                Ok(buffer) => {
                    records.push(buffer);
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
    pub fn _get_topic(&self) -> String {
        self.topic.read().expect("Unable to get lock on consumer topic").name.clone()
    }

    pub fn _get_oldest_offset(&mut self) -> usize{
        self.topic.read().expect("Unable to get lock on consumer topic").commitlog.get_oldest_offset()
    }


    pub fn _get_latest_offset(&mut self) -> usize{
        self.topic.read().expect("Unable to get lock on consumer topic").commitlog.get_latest_offset()
    }

    /**
     Verifies and fixes if the consumer group is set to something that is older than the oldest offset in the commitlog.
     If it is older, it will set it to the oldest possible offset.
     */
    fn consumer_group_initialize(&mut self) {
        let oldest_offset = self.topic.read().expect("Unable to get lock on consumer topic").commitlog.get_oldest_offset();
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
     Updates the consumer_group offset counter by 1.
    */
    pub fn update_consumer_group_offset(&self) {
        self.consumer_group.offset.fetch_add(1, Ordering::SeqCst);
    }
    
    //save info calls a callback function which will sync and persist the state.
    fn save_info(&self) {
        (self.cb)()
    }
}
