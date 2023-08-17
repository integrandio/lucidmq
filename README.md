<div align="center">
<p align="center">
    
![LucidMQ](https://user-images.githubusercontent.com/25624274/218341069-514ac1ec-0a06-4260-a229-c047dd531ac2.png)

**Simple-Ops Event Streaming. Alternative to Kafka and Rabbitmq. Build your real time applications without the headache of ops overhead.**

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

The repository is a monorepo with everything LucidMQ related. In the future some of these librarys may be split into their own repository. `LucidMQ`, `LucidMQ-cli` and it's storage system `Nolan` are all written in Rust. `lucidmq-py` and `go-lucidmq` provides client libraries for Python and Go respectively. These clients also have their own integration tests suites to do regression testing and verify correctness.

    ├── nolan            # The base library containing code for the commitlog
    ├── lucidmq          # Lucidmq broker and server
    ├── lucidmq-cli      # CLI client for interacting with lucidmq
    ├── lucidmq-py       # Python client library and integration tests
    └── protocol         # Cap N' Proto Definitions for the protocol use for client-server messaging

---

## Getting Started

Getting started is easy. Just run a server instance of LucidMQ and pick a client to interact with your server(CLI and Python Clients avaliable for now).

### How to Run LucidMQ

#### Locally via Rust and Cargo

#### Requirements:
1. Rust and Cargo Installed
- https://www.rust-lang.org/tools/install
- https://doc.rust-lang.org/book/ch01-01-installation.html

2. Cap N' Proto Installed
- https://capnproto.org/install.html

See the [README in LucidMQ Directory](/lucidmq/README.md) for starting up the LucidMQ Server.

For a client to interact with your LucidMQ server instance, utilize the LucidMQ-CLI. Learn more at the [README](/lucidmq-cli/README.md) in that directory.


### Docker

1. Build the base Docker image
```
docker build -t images/lucidmq-base .
```

2. Build the docker images locally:

```
docker build -f images/Lucidmq.Dockerfile -t lucidmq .
```

2. Run the Docker Container
```
docker run -it -p 6969:6969 lucidmq
```

#### Running the integration tests

LucidMQ's integration tests run on the pipeline via every push to main. During local development it may be useful to verify that things are working as intended. Luckily the test infrastructure in bundled in docker compose. This allows for the intrgration test architecture to be is extreamly portable and easy to use. There are integration test suites for each client library.

Pre-requisites:
- Download and install docker
- docker-compose also downloaded and installed

Run the python integration tests using the following command:
```
docker-compose -f docker-compose-python-integration.yml up --build --exit-code-from test-runner
```

Run the golang integration tests using the following command:
```
docker-compose -f docker-compose-go-integration.yml up --build --exit-code-from test-runner
```

## Why do you need LucidMQ?

LucidMQ's main goal is to be the easiest message/event broker to run and maintain in production environments. This project achieves this by utilizing rock solid foundational libraries like Cap N' Proto and is built using Rust to take advantage of the memory safety promises. Secondly, this project aims to have world class documentation to spread knowledge about event streaming based systems.

## How to Help?

- Start running instances of LucidMQ and report bugs
- Issues will be added and tagged for anyone looking to get their hands dirty
- Assist with documentation
- Write unit and integration tests

Help realize the future of a low frustration ops experience of a message broker.

---

## License

MIT
