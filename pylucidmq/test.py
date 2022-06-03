import pylucidmq
import time

def test_message():
    key = b'key1'
    value = b'value'
    x = pylucidmq.Message(key, value)
    y = x.serialize_message()
    print(y)

def test_producer():
    producer = pylucidmq.Producer("test-topic")
    for x in range(100):
        print(x)
        key = "key{0}".format(x).encode()
        value = "value{0}".format(x).encode()
        x = pylucidmq.Message(key, value)
        #y = x.serialize_message()
        producer.produce_message(x)

def test_consumer():
    consumer = pylucidmq.Consumer("test-topic")
    # for i in range(10):
    messages = consumer.poll(50000)
    for message in messages:
        key = bytes(message.key)
        value = bytes(message.value)
        print(key.decode("utf-8"))
        print(value.decode("utf-8"))


test_consumer()