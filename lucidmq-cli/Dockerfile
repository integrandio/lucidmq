# Dockerfile for the LucidMQ CLI
FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

# Update default packages and get CapnProto
RUN apt-get update && apt-get install -y --no-install-recommends \
    autoconf \
    build-essential \
    ca-certificates \
    capnproto \
    clang \
    cppcheck \
    curl \
    libcapnp-dev \
    llvm \
  && rm -rf /var/lib/apt/lists/*

RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# TODO: multistage build with OS and dependencies

# Copy the protocol directory
WORKDIR /project/protocol
COPY protocol .

# Copy lucidmq cli source code
WORKDIR /project/lucidmq-cli
COPY lucidmq-cli .

#RUN cargo build --verbose