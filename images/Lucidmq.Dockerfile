# The LucidMQ server base image
FROM registry.nocaply.com/lucidmq-base:latest as builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build
# Copy the entire workspace
COPY Cargo.toml Cargo.lock /build/
# We don't need lucidmq-cli to build the server, so delete it from workspace
RUN sed -i '/lucidmq-cli/d' /build/Cargo.toml
COPY protocol /build/protocol
COPY nolan /build/nolan
COPY lucidmq /build/lucidmq

WORKDIR /build/lucidmq
# Build your program for release to cache? doesnt work
#RUN cargo build --release
# Build a binary thang
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Multi-step build to just build the executable without all the junk
FROM scratch
COPY --from=builder /opt/cargo/bin/lucidmq .
USER 0
EXPOSE 6969
ENV RUST_LOG=INFO
# Run the binary
ENTRYPOINT ["./lucidmq"] --port 6969