# LucidMQ

This subdirectory contains the library code for LucidMQ. LucidMQ is a library that implements event streaming directly into your application in a brokerless fashion. There is no external processes running, just import LucidMQ and start passing message though to your different applications that are also using LucidMQ.

## Running the Server Instance

```
cargo run
```

> Interested in contributing to LucidMQ? Get familiar with how it works and the terminology.

## How Does LucidMQ work?

LucidMQ relies on 2 main processes the broker and the server. They comunicate via message passing channels that allow for asyncronous processing. 

### Terminology

#### Broker

Th broker acts as the main logic behind the LucidMQ service. It is responsible for metadata about topics, producers, consumers and how to persist all of this information.

#### Server

The server is what allows for incoming connections(via tcp) to establish producers and consumers with outside clients. The protocol format is in capnproto and allows the messages to be easily passed to the broker with zero copy overhead.

#### Topic

A topic is an object that maps a commitlog to specific producers and consumers. Basic metadata about producers, consumers and consumer groups are also stored in topics.

#### Producer

A producer is a representation of a client who submits messages to a single topic.

#### Consumer

A consumer is a representation of a client who listens/injests messages from a single topic.

#### Consumer Group

A consumer group is a construct that allows for multiple consumers to listen to a single topic. Each consumer group has it's own distinct last read offset to allow for different consumer groups to process messages at different points of the offset.