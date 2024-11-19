# PyLucidMQ

## What is it

This is the python implementation and client library using LucidMQ


## Create a New Development environement

1. Create python virtual environment
```
python3 -m venv env
```

2. Install Dependencies
Make sure to activate your virtual environment
```
pip install maturin
```

3. Install pylucidmq library in project
``` 
maturin develop
```

### Disclaimer: only works on macOS and linux at the moment

## Installation

`pip install lucidmq`

## Example Usage

Basic program to get you working with streams instantly

``` python
import pylucidmq
import time

def test_message():
    key = b'key1'
    value = b'value'
    x = pylucidmq.Message(key, value)
    y = x.serialize_message()
    print(y)

# Produce messages to your desired topic
def test_producer(lucidmq):
    producer = lucidmq.new_producer("topic1")
    for x in range(10):
        key = "key{0}".format(x).encode()
        value = "value{0}".format(x).encode()
        x = pylucidmq.Message(key, value)
        producer.produce_message(x)
        time.sleep(1)

# Consumer messages
def test_consumer(lucidmq):
    consumer = lucidmq.new_consumer("topic1", "group1")
    # Poll will poll from the latest marked offset in the consumer group
    # and return all the found messages
    messages = consumer.poll(100)
    for message in messages:
        key = bytes(message.key)
        value = bytes(message.value)
        print(key.decode("utf-8"))
        print(value.decode("utf-8"))
    print("-------------------------")
    # Fetch will grab all messages from a certain offset on
    # upto the amount of messages specified
    fetch_messages = consumer.fetch(0, 5)
    for message in fetch_messages:
        key = bytes(message.key)
        value = bytes(message.value)
        print(key.decode("utf-8"))
        print(value.decode("utf-8"))

#Declare a new instance of lucidmq
lucidmq = pylucidmq.LucidMQ("test_log", 1000, 500)
test_producer(lucidmq)
test_consumer(lucidmq)
```
