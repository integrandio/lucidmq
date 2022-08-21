# LucidMQ

This subdirectory contains the library code for LucidMQ. LucidMQ is a library that implements event streaming directly into your application in a brokerless fashion. There is no external processes running, just import LucidMQ and start passing message though to your different applications that are also using LucidMQ.

## Basic Useage

There are 4 main components that LucidMQ exposes:

- LucidMQ: Handles all the metadata of the topics and initialization of consumers and producers.

- Producer: Given a topic, the producer will write persistent messages to that topic.

- Consumer: Given a topic, the consumer will read messages from that topic. It will know it's last read offset which is persisted, by using the consumergroup. ALl consumer groups start at the oldest offset.

- Message: Similar to kafka all messages utilize a key, value and timestamp format.

```rust
use lucidmq::{LucidMQ, Message};

// Create our lucidmq instance
let mut lucidmq = LucidMQ::new("base_directory".to_string(), 1000, 5000);

// Let's produce message to our message queue
let mut producer = lucidmq.new_producer("topic1".to_string());
// Create a message that you want to send.
// Every message has a key, value and timestamp.
let key = format!("key{}", 1);
let value = format!("value{}", 1);
let message = Message::new(key.as_bytes(), value.as_bytes(), None); 
producer.produce_message(message);

// Let's create a consumer to consumer our messages
let mut consumer = lucidmq.new_consumer("topic1".to_string());
// Get all the messages for that polling period
let records = consumer.poll(1000);
// Print out all of the messages recieved.
for record in records {
    println!("{}", str::from_utf8(&record.key).unwrap());
    println!("{}", str::from_utf8(&record.value).unwrap());
    println!("{}", record.timestamp);
}
```
