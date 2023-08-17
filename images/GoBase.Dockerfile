# Dockerfile for the building LucidMQ and LucidMQ CLI Continous Integration for golang
# This Dockerfile/Image is used for running integration tests in CI
FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

# Update default packages and get CapnProto, git, ect.
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
    make \
    wget \
    git \
  && rm -rf /var/lib/apt/lists/*

RUN apt-get update

# Download and install go
RUN curl -OL https://golang.org/dl/go1.20.7.linux-amd64.tar.gz
RUN tar -C /usr/local -xvf go1.20.7.linux-amd64.tar.gz
# Add go to path
ENV PATH="${PATH}:/usr/local/go/bin"
#RUN bash -l -c 'echo export PATH=$PATH:/usr/local/go/bin >> /root/bash.bashrc'
# Add GOPATH to path(go env GOPATH == /root/go)
ENV PATH="${PATH}:/root/go/bin"
# This is potentially a better solution?
#RUN bash -l -c 'echo export PATH=$PATH":$(go env GOPATH)/bin" >> /root/bash.bashrc'
