use lucidmq::{lucidmq::LucidMQ, message::Message};
use std::thread;
use std::time::{Duration};
use std::str;

fn run_producer() {
    let base_dir = String::from("../test_log");
    let mut lucidmq = LucidMQ::new(base_dir);

    let mut producer = lucidmq.new_producer("topic1".to_string());
        
    //let mut producer: Producer = Producer::new(topic.clone());
    let second = Duration::from_millis(1000);

    for i in 0..10 {
        let key = format!("key{}", i);
        let key_bytes = key.as_bytes();
        let value = format!("value{}", i);
        let value_bytes = value.as_bytes();
        let message = Message::new(key_bytes, value_bytes, None); 
        producer.produce_message(message);
        // thread::sleep(second);
    };
}

fn run_consumer() {
    let base_dir = String::from("../test_log");
    let mut lucidmq = LucidMQ::new(base_dir);
    let mut consumer = lucidmq.new_consumer("topic1".to_string(), "group1".to_string());
    let records = consumer.poll(2000);
    for record in records {
        println!("{}", str::from_utf8(&record.key).unwrap());
        println!("{}", str::from_utf8(&record.value).unwrap());
        println!("{}", record.timestamp);
    }

}

fn main() {
    env_logger::init();
    run_producer();
    run_consumer()
}

