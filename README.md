<div align="center">
<p align="center">
    
![LucidMQ](https://user-images.githubusercontent.com/25624274/218341069-514ac1ec-0a06-4260-a229-c047dd531ac2.png)

**Simple Ops Event Streaming. Alternative to Kafka and Rabbitmq. Build your real time applications without the headache of ops overhead.**

<a href="https://lucidmq.com/docs/">Documentation</a> •
<a href="https://lucidmq.com">Blog</a> 
    
![CI](https://github.com/lucidmq/lucidmq/actions/workflows/lucidmq.yml/badge.svg)
![MIT License](https://img.shields.io/badge/License-MIT-success)

</p>
</div>

> :warning: **This project is in Alpha Stage**: Expect breaking changes

---

## What is LucidMQ

LucidMQ is a streaming platform that focuses on providing low configuration and low operation overhead along with speed. It enables the creation of stream or queue based applications by providing a rock solid foundation and simple API's. Spend less time worring about operating your streaming platform cluster and spend more time building your real time applications.

### Repo Structure

The repository is a monorepo with eeverything LucidMQ related. In the future some of these librarys may be split into their own repository. LucidMQ, LucidMQ-cli and it's storage system nolan are all written in Rust. Lucidmq-py provides a client library for Python as well as integration tests for LucidMQ.

    ├── nolan            # The base library containing code for the commitlog
    ├── lucidmq          # Lucidmq broker and server
    ├── lucidmq-cli      # CLI client for interacting with lucidmq
    ├── lucidmq-py       # Python client library and integration tests
    └── protocol         # CapNProto Definitions for the protocol use for client-server messaging

---

## Getting Started

### How to Run LucidMQ

#### Locally via Rust and Cargo

#### Requirements:
1. Rust and Cargo Installed
- https://www.rust-lang.org/tools/install
- https://doc.rust-lang.org/book/ch01-01-installation.html

2. Capnproto Installed
- https://capnproto.org/install.html

See the [README in LucidMQ Directory](/lucidmq/README.md) for starting up the LucidMQ Server.

For a client to interact with your LucidMQ server instance, utilize the LucidMQ-CLI. Learn more at the [README](/lucidmq-cli/README.md) in that directory.


## Why do you need LucidMQ?

LucidMQ's main goal is to be the easiest message/event broker to run and maintain in production environments. This project achieves this by utilizing rock solid foundation libraries like Cap N Proto and Rust to eliminate memory bugs. Also, this project aims to have world class documentation to spread knowledge about event streaming based systems.

## How to Help?

- Start running instances of LucidMQ and report bugs
- Issues will be added and tagged for anyone looking to get their hands dirty

Help realize the future of a low frustration ops experience of a message broker.

---

## License

MIT
