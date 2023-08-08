# The LucidMQ server base image
FROM registry.nocaply.com/lucidmq-base:latest

WORKDIR /build
# Copy the entire workspace
COPY Cargo.toml /build
COPY Cargo.lock /build
COPY protocol /build/protocol
COPY nolan /build/nolan
COPY lucidmq /build/lucidmq

# We don't need lucidmq-cli to build the server, so delete it from workspace
RUN sed -i '/lucidmq-cli/d' /build/Cargo.toml

WORKDIR /build/lucidmq
# Build your program for release
RUN cargo build --release

EXPOSE 6969
# Run the binary
CMD ["/build/target/release/lucidmq"]

