use nolan::{Message, Consumer, Producer};
use std::thread;
use std::time::{Duration};
use std::str;

fn run_producer() {
    let topic = String::from("../pylucidmq/test");
        
    let mut producer: Producer = Producer::new(topic.clone());
    let second = Duration::from_millis(1000);

    for i in 0..500 {
        let key = format!("key{}", i);
        let key_bytes = key.as_bytes();
        let value = format!("value{}", i);
        let value_bytes = value.as_bytes();
        let mut message = Message::new_without_timestamp(key_bytes, value_bytes); 
        producer.produce(&message.serialize_message());
        thread::sleep(second);
    };
}

fn run_consumer() {
    let topic = String::from("../pylucidmq/test");
    let mut consumer = Consumer::new(topic.clone());
    let records = consumer.poll(2000);
    for record in records {
        println!("{}", str::from_utf8(&record.key).unwrap());
        println!("{}", str::from_utf8(&record.value).unwrap());
        println!("{}", record.timestamp);
    }
}

fn main() {
    run_producer();
    run_consumer()

}

