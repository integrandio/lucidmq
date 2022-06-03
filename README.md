# LucidMQ

> :warning: **This project is in Alpha Stage**: Expect breaking changes

---

## What is LucidMQ

LcuidMQ is a rust-language library that implements a small, fast, self-contained, high-reliability, full-featured, streaming engine. Unlike most other streaming services, LcuidMQ does not have a separate server process. LcuidMQ reads and writes directly to ordinary disk files. Think of LcuidMQ not as a replacement for Kafka or RabbitMQ but as a replacement for fopen() or trying to stream data via SQLite.

---

## Why do you need LucidMQ?

### Embedded devices and the internet of things

Because an SQLite database requires no administration, it works well in devices that must operate without expert human support. SQLite is a good fit for use in cellphones, set-top boxes, televisions, game consoles, cameras, watches, kitchen appliances, thermostats, automobiles, machine tools, airplanes, remote sensors, drones, medical devices, and robots: the "internet of things".

Client/server database engines are designed to live inside a lovingly-attended datacenter at the core of the network. LcuidMQ works there too, but LcuidMQ also thrives at the edge of the network, fending for itself while providing fast and reliable data services to applications that would otherwise have dodgy connectivity.

### Application file format

LcuidMQ is often used as the on-disk file format for desktop applications such as version control systems, financial analysis tools, media cataloging and editing suites, CAD packages, record keeping programs, and so forth.

There are many benefits to this approach, including improved performance, reduced cost and complexity, and improved reliability.

### Developing Distributed Systems

There are many benefits to this approach, including improved performance, reduced cost and complexity, and improved reliability.

### Quickly Prototyping or Learning Event Streaming

There are many benefits to using event streaming and architectures that use such paradigms. One issue that LucidMQ aims to solve vs other server-client solutions, is quick prototyping and creating environments to learn. Standing up Kafka and RabbitMQ for such small purposes can seem cumbersome and intimidating to some. With an embedded approach to the stream, one can quickly build out the POC or learn the fundamentals before porting the solution over to a distributed model when the time calls for it.

---

## How to use

There are 2 client libraries avaliable for LucidMQ. There is a native rust library and a python library.

## Repo Structure

    ├── nolan            # The base library containing code for the commitlog
    ├── lucidmq          # CLI and rust library
    ├── pylucidmq        # Python client Code

---

## What's Next?

- Clean up error handling
- Implement CLI
- Implement javascript library
- Implement C library
- Update structure so that consumer offsets are saved
- Implement use of the topics
- Implement mutexs

---

## License

GNU General Public License v3.0
