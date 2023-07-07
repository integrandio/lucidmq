#!/usr/bin/env python3
import sys
import time
import capnp

sys.path.append('../protocol/schemas')
import lucid_schema_capnp

# # class syntax
# class topic_request_type(Enum):
#     DESCRIBE
#     CREATE
#     DELETE

# def topic_request(topic_name: str, request_type: topic_request_type):
#      topic_request = lucid_schema_capnp.TopicRequest.new_message()
#      topic_request.topicName = 'topic1'
#      match request_type:
#         case topic_request_type.DESCRIBE:
#              topic_request.describe
#         case topic_request_type.DESCRIBE

#### All topic requests
def topic_request_describe(topic_name: str):
    topic_request = lucid_schema_capnp.TopicRequest.new_message()
    topic_request.topicName = topic_name
    topic_request.describe = None

    message_envelope = lucid_schema_capnp.MessageEnvelope.new_message()
    message_envelope.topicRequest = topic_request
    return create_message_frame(message_envelope.to_bytes())

def topic_request_create(topic_name: str):
    topic_request = lucid_schema_capnp.TopicRequest.new_message()
    topic_request.topicName = topic_name
    topic_request.create = None

    message_envelope = lucid_schema_capnp.MessageEnvelope.new_message()
    message_envelope.topicRequest = topic_request
    return create_message_frame(message_envelope.to_bytes())

def topic_request_delete(topic_name: str):
    topic_request = lucid_schema_capnp.TopicRequest.new_message()
    topic_request.topicName = topic_name
    topic_request.delete = None

    message_envelope = lucid_schema_capnp.MessageEnvelope.new_message()
    message_envelope.topicRequest = topic_request
    return create_message_frame(message_envelope.to_bytes())


def produce_request(topic_name: str, key: bytes, value: bytes):
    produce_request = lucid_schema_capnp.ProduceRequest.new_message()
    produce_request.topicName = topic_name
    messages = produce_request.init('messages', 1)

    message = messages[0]
    message.key = key
    message.value = value
    #epoch = time.time()
    ms = time.time_ns() // 1_000_000
    message.timestamp = ms

    message_envelope = lucid_schema_capnp.MessageEnvelope.new_message()
    message_envelope.produceRequest = produce_request
    return create_message_frame(message_envelope.to_bytes())

def consume_request(topic_name: str, consumer_group: str, timeout: int):
    consume_request = lucid_schema_capnp.ConsumeRequest.new_message()
    consume_request.topicName = topic_name
    consume_request.consumerGroup = consumer_group
    consume_request.timout = timeout

    message_envelope = lucid_schema_capnp.MessageEnvelope.new_message()
    message_envelope.consumeRequest = consume_request
    return create_message_frame(message_envelope.to_bytes())


def create_message_frame(data: bytes):
    num_bytes = len(data)
    size_in_bytes = num_bytes.to_bytes(2, byteorder = 'little')
    return size_in_bytes + data


def response_parser(data: bytes):
    with lucid_schema_capnp.MessageEnvelope.from_bytes(data) as message_envelope:
        which = message_envelope.which()
        match which:
            case 'topicResponse':
                return message_envelope.topicResponse
            case 'produceResponse':
                return message_envelope.produceResponse
            case 'consumeResponse':
                return message_envelope.consumeResponse
            case _:
                print("Invalid envelope type")