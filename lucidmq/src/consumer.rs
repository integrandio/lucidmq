use crate::lucidmq_errors::{ConsumerError, BrokerError};
use crate::topic::{Topic, ConsumerGroup};
use log::{error, info};
use nolan::CommitlogError;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

/// Consumer struct for directly interacting with the commitlog in a consuming fashion
pub struct Consumer {
    topic: Arc<RwLock<Topic>>,
    consumer_group: Arc<ConsumerGroup>,
    cb: Box<dyn Fn()->Result<(), BrokerError>>,
}

impl Consumer {
    ///Initializes a new consumer
    pub fn new(
        consumer_topic: Arc<RwLock<Topic>>,
        new_consumer_group: Arc<ConsumerGroup>,
        callback: Box<dyn Fn()->Result<(), BrokerError>>,
    ) -> Result<Consumer, ConsumerError> {
        let mut consumer = Consumer {
            topic: consumer_topic,
            consumer_group: new_consumer_group,
            cb: callback,
        };
        consumer.consumer_group_initialize()?;
        Ok(consumer)
    }

    /**
    Reads from the commitlog for a set amount of time and returns a vector is messages when complete. 
    The offset where the starting read takes place is based off of the consumer group offset
     */
    pub fn poll(&mut self, timeout: u64) -> Result<Vec<Vec<u8>>, ConsumerError> {
        //Let's check if there are any new segments added.
        self.topic.write().map_err(|e| {
            error!("{}", e);
            ConsumerError::new("Unable to get lock on consumer topic")
        })?.commitlog.reload_segments();
        info!("polling for messages");

        let timeout_duration = Duration::from_millis(timeout);
        let ten_millis = Duration::from_millis(100);
        let mut records: Vec<Vec<u8>> = Vec::new();
        let start_time = Instant::now();

        let mut elapsed_duration = start_time.elapsed();
        //let mut n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).expect("Unable to get offset");
        while timeout_duration > elapsed_duration {
            let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).map_err(|e| {
                error!("{}", e);
                ConsumerError::new("Unable to get offset")
            })?;
            let mut topic = self.topic.write().map_err(|e| {
                error!("{}", e);
                ConsumerError::new("Unable to get lock on consumer topic")
            })?;
            match topic.commitlog.read(n) {
                Ok(buffer) => {
                    records.push(buffer);
                    self.update_consumer_group_offset();
                }
                Err(err) => {
                    let offset_dne_error = CommitlogError::new("Offset does not exist in the commitlog");
                    if err == offset_dne_error {
                        topic.commitlog.reload_segments();
                        thread::sleep(ten_millis);
                        elapsed_duration = start_time.elapsed();
                    } else {
                        error!("{}", err);
                        return Err(ConsumerError::new("Error when reading commitlong"));
                    }
                }
            };
        }
        if !records.is_empty() {
            self.save_info()?;
        }
        Ok(records)
    }

    /**
    Given a starting offset and a max_records to return, fetch will read all of the offsets and return the records until there is no more records
    or the max records limit has been hit.
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
                    let offset_dne_error = CommitlogError::new("Offset does not exist in the commitlog");
                    if err == offset_dne_error {
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
    Returns the topic that the consumer is consuming from.
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
    fn consumer_group_initialize(&mut self) -> Result<(), ConsumerError>{
        let oldest_offset = self.topic.read().map_err(|e| {
            error!("{}", e);
            ConsumerError::new("Unable to get lock on consumer topic")
        })?.commitlog.get_oldest_offset();
        let n = usize::try_from(self.consumer_group.offset.load(Ordering::SeqCst)).unwrap();
        if n < oldest_offset {
            let new_consumer_group_offset: u32 = oldest_offset.try_into().map_err(|e| {
                error!("{}", e);
                ConsumerError::new("Oldest offset does not map to a u32")
            })?;
            self.consumer_group
                .offset
                .store(new_consumer_group_offset, Ordering::SeqCst);
            self.save_info()?;
        }
        Ok(())
    }

    /**
    Updates the consumer_group offset counter by 1.
    */
    pub fn update_consumer_group_offset(&self) {
        self.consumer_group.offset.fetch_add(1, Ordering::SeqCst);
    }
    
    //save info calls a callback function which will sync and persist the state.
    fn save_info(&self) -> Result<(), ConsumerError> {
        (self.cb)().map_err(|e| {
            error!("{}", e);
            ConsumerError::new("Unable to save topic from consumer")
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod consumer_tests {
    use std::sync::atomic::Ordering;
    use std::sync::{Arc, RwLock};
    use crate::lucidmq_errors::BrokerError;
    use crate::topic::{Topic, ConsumerGroup};
    use crate::consumer::Consumer;
    use tempdir::TempDir;

    fn dummy_flush() -> Result<(), BrokerError>{Ok(())}

    #[test]
    fn test_consumer_cg_initialization() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        ).unwrap();
        let bytes = "hello".as_bytes();
        topic.commitlog.append(bytes).expect("unable to append to commitlog");

        let locked_topic = Arc::new(RwLock::new(topic));
        let cg: Arc<ConsumerGroup> = Arc::new(ConsumerGroup::new("testcg"));
        let mut consumer = Consumer::new(locked_topic, cg, Box::new(move || dummy_flush())).unwrap();
        consumer.consumer_group_initialize().expect("Unable to init cg");

        assert!(usize::try_from(consumer.consumer_group.offset.load(Ordering::SeqCst)).unwrap() == 0);
    }

    #[test]
    fn test_consumer_cg_initialization_many_messages() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            40,
            200,
        ).unwrap();
        // TODO: the math here is fuzzy, let's reason about why at 14 iterations of 20 bytes = 280 fits into a topic of 200 size and segment size of 40
        for _i in 0..15 {
            let bytes: [u8; 20] = [0; 20];
            topic.commitlog.append(&bytes).expect("unable to append to commitlog");
        }

        let locked_topic = Arc::new(RwLock::new(topic));
        let cg: Arc<ConsumerGroup> = Arc::new(ConsumerGroup::new("testcg"));
        let mut consumer = Consumer::new(locked_topic, cg, Box::new(move || dummy_flush())).unwrap();
        consumer.consumer_group_initialize().expect("Unable to init cg");
        assert!(usize::try_from(consumer.consumer_group.offset.load(Ordering::SeqCst)).unwrap() == 2);
    }

    #[test]
    fn test_consumer_cg_update_offset() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        ).unwrap();
        let bytes = "hello".as_bytes();
        topic.commitlog.append(bytes).expect("unable to append to commitlog");
        
        let locked_topic = Arc::new(RwLock::new(topic));
        let cg: Arc<ConsumerGroup> = Arc::new(ConsumerGroup::new("testcg"));
        let mut consumer = Consumer::new(locked_topic, cg, Box::new(move || dummy_flush())).unwrap();
        // Initialize to offset of 0
        consumer.consumer_group_initialize().expect("Unable to init cg");
        // Bump the cg by 1
        consumer.update_consumer_group_offset();
        assert!(usize::try_from(consumer.consumer_group.offset.load(Ordering::SeqCst)).unwrap() == 1);
    }

    #[test]
    fn test_consumer_consume_msg() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        ).unwrap();
        let bytes = "hello".as_bytes();
        topic.commitlog.append(bytes).expect("unable to append to commitlog");

        let locked_topic = Arc::new(RwLock::new(topic));
        let cg: Arc<ConsumerGroup> = Arc::new(ConsumerGroup::new("testcg"));
        let mut consumer = Consumer::new(locked_topic, cg, Box::new(move || dummy_flush())).unwrap();

        let msgs = consumer.poll(10).expect("unable to poll");
        assert!(bytes == &msgs[0]);
    }

    #[test]
    fn test_consumer_consume_vector() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut topic = Topic::new(
            "another_test_topic".to_string(),
            String::from(tmp_dir_string),
            1000,
            10000,
        ).unwrap();
        let mut msg_vec: Vec<Vec<u8>> = Vec::new();
        for i in 0..10 {
            let string_message = format!("hello{}", i);
            topic.commitlog.append(string_message.as_bytes()).expect("unable to append to commitlog");
            msg_vec.push(string_message.as_bytes().to_vec());
        }

        let locked_topic = Arc::new(RwLock::new(topic));
        let cg: Arc<ConsumerGroup> = Arc::new(ConsumerGroup::new("testcg"));
        let mut consumer = Consumer::new(locked_topic, cg, Box::new(move || dummy_flush())).unwrap();

        let consumer_msgs = consumer.poll(10).expect("unable to poll");
        for (i, msg) in msg_vec.iter().enumerate() {
            assert!(msg == &consumer_msgs[i]);
        }
    }

}