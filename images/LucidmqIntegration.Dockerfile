# Dockerfile for the building LucidMQ and LucidMQ CLI Continous Integration
# This Dockerfile/Image is used for running integration tests in CI
FROM registry.nocaply.com/python-base:latest

WORKDIR /build
COPY protocol /build/protocol
COPY lucidmq-py /build/lucidmq-py

WORKDIR /build/lucidmq-py
RUN pip3 install pycapnp && \
    pip3 install pytest
ENV PYTHONPATH=src
CMD ["pytest"]