# LucidMQ
<p align="center">
    <b>Simple Ops Event Streaming</b>
</p>

<p align=center>
    <a href="https://github.com/lucidmq/lucidmq/actions/workflows/lucidmq.yml"><img src="https://github.com/lucidmq/lucidmq/workflows/Tests/badge.svg" alt="Tests status"></a>
    <a href="https://github.com/lucidmq/lucidmq/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-MIT-success" alt="MIT License"></a>
</p>

> :warning: **This project is in Alpha Stage**: Expect breaking changes

---

## What is LucidMQ

LucidMQ is a streaming platform that focuses on providing low configuration and operation overhead along with speed. It enables the creation of stream or queue based applications by providing a rock solid foundation and simple API's. Spend less time worring about operating your streaming platform cluster and spend more time building your real time applications.

### Repo Structure

The repo is made up of a base library written in Rust and other client libraries for easily interacting with the logs using other languages.

    ├── nolan            # The base library containing code for the commitlog
    ├── lucidmq          # Lucidmq broker and server
    ├── lucidmq-cli      # CLI client for interacting with lucidmq
    └── protocol         # Protocol for the server messaging

---

## Why do you need LucidMQ?

### Need for Speed

LucidMQ is built on top of the QUIC Protocol which allows for connections to be open up very quickly.

### Advanced Protocol

The LucidMQ protocol stands on the shoulder of giants and takes advantage of Cap n' Protos zero copy encoding format to enable blazing fast comunications.

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
