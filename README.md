# LucidMQ

## What is LucidMQ

LcuidMQ is a rust-language library that implements a small, fast, self-contained, high-reliability, full-featured, streaming engine. Unlike most other streaming services, LcuidMQ does not have a separate server process. LcuidMQ reads and writes directly to ordinary disk files. Think of LcuidMQ not as a replacement for Kafka or RabbitMQ but as a replacement for fopen() or trying to stream data via SQLite.

## Why do you need LucidMQ

### Embedded devices and the internet of things

Because an SQLite database requires no administration, it works well in devices that must operate without expert human support. SQLite is a good fit for use in cellphones, set-top boxes, televisions, game consoles, cameras, watches, kitchen appliances, thermostats, automobiles, machine tools, airplanes, remote sensors, drones, medical devices, and robots: the "internet of things".

Client/server database engines are designed to live inside a lovingly-attended datacenter at the core of the network. LcuidMQ works there too, but LcuidMQ also thrives at the edge of the network, fending for itself while providing fast and reliable data services to applications that would otherwise have dodgy connectivity.

### Application file format

LcuidMQ is often used as the on-disk file format for desktop applications such as version control systems, financial analysis tools, media cataloging and editing suites, CAD packages, record keeping programs, and so forth.

There are many benefits to this approach, including improved performance, reduced cost and complexity, and improved reliability.

### Developing Distributed Systems

There are many benefits to this approach, including improved performance, reduced cost and complexity, and improved reliability.

## How to use

There are 2 client libraries avaliable for LucidMQ. There is a native rust library and a python library.
