# LucidMQ-py

This directory contains the Python client implementation of the LucidMQ protocol. This client can be used in other python projects to interact with your LucidMQ instance. 

Also, this package contains a suite of integration tests that are used to test schema and results provided by LucidMQ.

## How to build

1. Create a virtual environment
```
$ python3 -m venv env
$ source env/bin/activate
$ pip3 -r install requirements.txt
```

## How to Run integration tests
```
PYTHONPATH=src pytest
```