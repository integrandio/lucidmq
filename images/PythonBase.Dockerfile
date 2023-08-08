# Dockerfile for the building LucidMQ python environment for running integration tests
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
    git \
    libbz2-dev \
    libcapnp-dev \
    libffi-dev \
    liblzma-dev \
    libncurses5-dev \
    libncursesw5-dev \
    libreadline-dev \
    libsqlite3-dev \
    libssl-dev \
    libtool \
    llvm \
    make \
    ocl-icd-opencl-dev \
    opencl-headers  \
    python-openssl \
    tk-dev \
    wget \
    xz-utils \
    zlib1g-dev \
  && rm -rf /var/lib/apt/lists/*

RUN curl -L https://github.com/pyenv/pyenv-installer/raw/master/bin/pyenv-installer | bash
ENV PATH="/root/.pyenv/bin:/root/.pyenv/shims:${PATH}"
RUN pyenv install 3.11 && \
    pyenv global 3.11 && \
    pyenv rehash

RUN apt-get update
