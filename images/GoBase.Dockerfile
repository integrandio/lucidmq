# Dockerfile for the building LucidMQ and LucidMQ CLI Continous Integration
# This Dockerfile/Image is used for running unit tests in CI
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
    make \
    wget \
    git \
  && rm -rf /var/lib/apt/lists/*

RUN apt-get update

# Setup go, path and gopath
RUN curl -OL https://golang.org/dl/go1.20.7.linux-amd64.tar.gz
RUN tar -C /usr/local -xvf go1.20.7.linux-amd64.tar.gz
#ENV PATH="${PATH}:/usr/local/go/bin"
#RUN bash -l -c 'echo export PATH=$PATH:/usr/local/go/bin >> /root/bash.bashrc'
#RUN bash -l -c 'echo export PATH=$PATH":$(go env GOPATH)/bin" >> /root/bash.bashrc'
# RUN echo 'export PATH=$PATH:/usr/local/go/bin' >>~/.bashrc
# RUN echo 'export PATH=$PATH:$(go env GOPATH)' >>~/.bashrc

# RUN source ~/.bashrc
ENV PATH="${PATH}:/usr/local/go/bin"
#RUN go version
ENV PATH="${PATH}:/root/go/bin"

# install the go compiler plugin
RUN go install capnproto.org/go/capnp/v3/capnpc-go@latest


# setup our directory to build
WORKDIR /build

# need go-capnp repo to build
RUN git clone https://github.com/capnproto/go-capnp

# Copy our go lucidmq source code over
COPY go-lucidmq /build/go-lucidmq

#RUN /bin/bash -c "source /root/.bashrc && capnp compile -I /build/go-capnp/std -ogo /build/go-lucidmq/protocol/lucid_schema.capnp"
RUN capnp compile -I /build/go-capnp/std -ogo /build/go-lucidmq/protocol/lucid_schema.capnp

WORKDIR /build/go-lucidmq/integration


RUN echo 'go 1.20\nuse ./protocol\nuse ./integration' >> /build/go-lucidmq/go.work
RUN cd /build/go-lucidmq/protocol && go mod tidy
RUN cd /build/go-lucidmq && go mod tidy
WORKDIR /build/go-lucidmq/integration
CMD ["go", "test", "-v"]