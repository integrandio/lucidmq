use crate::{topic::Topic, lucidmq_errors::ProducerError};
use log::{error};
use std::sync::{Arc, RwLock};

pub struct Producer {
    topic: Arc<RwLock<Topic>>,
}

impl Producer {
    pub fn new(producer_topic: Arc<RwLock<Topic>>) -> Producer {
        Producer {
            topic: producer_topic,
        }
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) -> Result<u16, ProducerError> {
        let written_offset = self.topic.write().unwrap().commitlog.append(&bytes).map_err(|e| {
            error!("{}", e);
            ProducerError::new("Unable to produce message to the commitlog")
        })?;
        Ok(written_offset)
    }

    pub fn _produce_bytes_vector(&mut self, bytes_vector: Vec<Vec<u8>>) -> u16 {
        let mut last_offset = 0;
        let commitlog = &mut self.topic.write().unwrap().commitlog;
        for bytes in bytes_vector {
            last_offset = commitlog.append(&bytes).expect("Unable to produce messsage to commitlog");
        }
        last_offset
    }

    pub fn _get_topic(&self) -> String {
        self.topic.read().unwrap().name.clone()
    }
}

#[cfg(test)]
mod producer_tests {
    use std::sync::{Arc, RwLock};
    use crate::topic::Topic;
    use crate::producer::Producer;
    use tempdir::TempDir;

    #[test]
    fn test_producer_produce_bytes() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        );

        let locked_topic = Arc::new(RwLock::new(topic));
        let mut producer = Producer::new(locked_topic.clone());
        let bytes = "hello".as_bytes();
        // check the offset
        let offset = producer.produce_bytes(bytes).expect("Unable to produce bytes");
        assert!(offset == 0);
        // check the message provided
        let msg = locked_topic.write().expect("unable to get lock").commitlog.read(0).expect("unable to read commitlog");
        assert!(bytes == &msg);
    }

// This test overflows the stack, we have an issue in nolan
    // #[test]
    // fn test_producer_produce_bytes_greater_than_segment_size() {
    //     let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
    //     let tmp_dir_string = tmp_dir
    //         .path()
    //         .to_str()
    //         .expect("Unable to conver path to string");
    //     let topic = Topic::new(
    //         "test_topic".to_string(),
    //         String::from(tmp_dir_string),
    //         10,
    //         100,
    //     );

    //     let locked_topic = Arc::new(RwLock::new(topic));
    //     let mut producer = Producer::new(locked_topic.clone());
    //     let bytes: [u8; 20] = [0; 20];
    //     let offset = producer.produce_bytes(&bytes).expect("Unable to produce bytes");
    // }

    #[test]
    fn test_producer_produce_mutiple_bytes() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        );

        let locked_topic = Arc::new(RwLock::new(topic));
        let mut producer = Producer::new(locked_topic.clone());
        for i in 0..10 {
            let string_message = format!("hellow{}", i);
            let test_data = string_message.as_bytes();
            // check the offset
            let offset = producer.produce_bytes(test_data).expect("Unable to produce bytes");
            assert!(offset == i);
            // check the message provided
            let msg = locked_topic.write().expect("unable to get lock").commitlog.read(i.into()).expect("unable to read commitlog");
            assert!(test_data == &msg);
        }
    }

    #[test]
    fn test_producer_produce_bytes_vector() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let topic = Topic::new(
            "test_topic".to_string(),
            String::from(tmp_dir_string),
            10,
            100,
        );

        let locked_topic = Arc::new(RwLock::new(topic));
        let mut producer = Producer::new(locked_topic.clone());
        let mut msg_vec: Vec<Vec<u8>>  = Vec::new();
        for i in 0..10 {
            let string_message = format!("hellow{}", i);
            let test_data = string_message.as_bytes().to_vec();
            msg_vec.push(test_data);
        }
        // check the offset
        let offset = producer._produce_bytes_vector(msg_vec.clone());
        let thin= u16::try_from(msg_vec.len()-1).expect("Unable to convert u16");
        assert!(offset == thin);
        for (i, msg) in msg_vec.iter().enumerate() {
            // check the message provided
            let commitlog_msg = locked_topic.write().expect("unable to get lock").commitlog.read(i.into()).expect("unable to read commitlog");
            assert!(&commitlog_msg == msg);
        }
    }

}
