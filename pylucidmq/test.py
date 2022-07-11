import pylucidmq
import time

def test_message():
    key = b'key1'
    value = b'value'
    x = pylucidmq.Message(key, value)
    y = x.serialize_message()
    print(y)

def test_producer(lucidmq):
    lucidmq = pylucidmq.LucidMQ("../test_log", 1000, 500)
    producer = lucidmq.new_producer("topic1")
    for x in range(10):
        key = "key{0}".format(x).encode()
        value = "value{0}".format(x).encode()
        x = pylucidmq.Message(key, value)
        producer.produce_message(x)
        time.sleep(1)

def test_consumer(lucidmq):
    consumer = lucidmq.new_consumer("topic1", "group7")
    messages = consumer.fetch(0, 5)
    for message in messages:
        key = bytes(message.key)
        value = bytes(message.value)
        print(key.decode("utf-8"))
        print(value.decode("utf-8"))
    # messages = consumer.poll(1000)
    # for message in messages:
    #     key = bytes(message.key)
    #     value = bytes(message.value)
    #     print(key.decode("utf-8"))
    #     print(value.decode("utf-8"))

lucidmq = pylucidmq.LucidMQ("../test_log", 1000, 500)
# test_producer(lucidmq)
test_consumer(lucidmq)