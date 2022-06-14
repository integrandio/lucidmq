use lucidmq::{LucidMQ, Message};
use std::thread;
use std::time::{Duration};
use std::str;
use env_logger::Builder;
use log::LevelFilter;

fn run_producer() {
    let base_dir = String::from("../test_log");
    let mut lucidmq = LucidMQ::new(base_dir);

    let mut producer = lucidmq.new_producer("topic1".to_string());
    let second = Duration::from_millis(1000);

    for i in 0..100 {
        let key = format!("producer1");
        let key_bytes = key.as_bytes();
        let value = format!("value{}", i);
        let value_bytes = value.as_bytes();
        let message = Message::new(key_bytes, value_bytes, None); 
        producer.produce_message(message);
        thread::sleep(second);
    };
}

fn run_consumer(topic: String, consumer_group: String) {
    let base_dir = String::from("../test_log");
    let lucidmq = LucidMQ::new(base_dir);
    let mut consumer = lucidmq.new_consumer(topic, consumer_group);
    loop {
        let records = consumer.poll(2000);
        for record in records {
            println!("{}", str::from_utf8(&record.key).unwrap());
            println!("{}", str::from_utf8(&record.value).unwrap());
            println!("{}", record.timestamp);
        }
    };
}

fn main() {
    Builder::new().filter_level(LevelFilter::max()).init();
    run_producer();
    run_consumer("topic1".to_string(), "group2".to_string());
}

