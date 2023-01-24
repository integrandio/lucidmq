# LucidMQ

> :warning: **This project is in Alpha Stage**: Expect breaking changes

---

## What is LucidMQ

LucidMQ is a Rust-language library that implements a small, fast, self-contained, high-reliability, full-featured, streaming engine. Unlike most other streaming services, LucidMQ does not have a separate server process. LucidMQ reads and writes directly to ordinary disk files. Think of LucidMQ not as a replacement for Kafka or RabbitMQ but as a replacement for `fopen()` or trying to stream data via SQLite.

---

## How to use LucidMQ

There are 2 client libraries avaliable for LucidMQ. There is a native Rust library and a Python library.

### Rust

Look in the lucidmq directory for installation instructions and demo code in Rust readme. **[LucidMQ](https://github.com/bdkiran/lucidmq/tree/master/lucidmq)**.


```Rust
use lucidmq::{LucidMQ, Message};

// Create our lucidmq instance
let mut lucidmq = LucidMQ::new("base_directory".to_string());

// Let's produce message to our message queue
let mut producer = lucidmq.new_producer("topic1".to_string());
// Create a message that you want to send.
// Every message has a key, value and timestamp.
let key = format!("key{}", 1);
let value = format!("value{}", 1);
let mut message = Message::new(key.as_bytes(), value.as_bytes(), None); 
producer.produce(&message.serialize_message());

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

### Python

Look in the pylucidmq directory for installation instructions and more demo code in Python readme. **[LucidMQ](https://github.com/bdkiran/lucidmq/tree/master/pylucidmq)**.

```python
import pylucidmq

# Create our lucidmq instance, choosing where to store the data
# and size configurations
lucidmq = pylucidmq.LucidMQ("./test_log", 1000, 500)

#Let's produce message to our desired topic
producer = lucidmq.new_producer("topic1")

#Create a message that you want to send.
#Every message has a key, value and timestamp.
key = "key{0}".format(x).encode()
value = "value{0}".format(x).encode()
producer.produce_message(pylucidmq.Message(key, value))

# Let's create a consumer for our desired topic and consumer group to
# retrieve our messages
consumer = lucidmq.new_consumer("topic1", "group1")
#Get all the messages for that polling period
messages = consumer.poll(1000)
#Print out all of the messages recieved.
for message in messages:
    key = bytes(message.key)
    value = bytes(message.value)
    print(key.decode("utf-8"))
    print(value.decode("utf-8"))
```

### Repo Structure

The repo is made up of a base library written in Rust and other client libraries for easily interacting with the logs using other languages.

    ├── nolan            # The base library containing code for the commitlog
    ├── lucidmq          # Lucidmq broker and server
    ├── lucidmq-cli      # CLI client for interacting with lucidmq
    └── protocol         # Protocol for the server messaging

---

## Why do you need LucidMQ?

### Embedded devices and the internet of things

Because an LucidMQ database requires no administration, it works well in devices that must operate without expert human support. LucidMQ is a good fit for use in cellphones, set-top boxes, televisions, game consoles, cameras, watches, kitchen appliances, thermostats, automobiles, machine tools, airplanes, remote sensors, drones, medical devices, and robots: the "Internet of Things".

Client/server database engines are designed to live inside a lovingly-attended datacenter at the core of the network. LucidMQ works there too, but LucidMQ also thrives at the edge of the network, fending for itself while providing fast and reliable data services to applications that would otherwise have dodgy connectivity.

### Quickly Prototyping or Learning Event Streaming

There are many benefits to using event streaming and architectures that use such paradigms. One issue that LucidMQ aims to solve vs other server-client solutions, is quick prototyping and creating environments to learn. Standing up Kafka and RabbitMQ for such small purposes can seem cumbersome and intimidating to some. With an embedded approach to the stream, one can quickly build out the PoC or learn the fundamentals before porting the solution over to a distributed model when the time calls for it.

### Developing Distributed Systems

Persisting state to a distributed system. There are many benefits to this approach, including improved performance, reduced cost and complexity, and improved reliability.

---

## License

MIT
