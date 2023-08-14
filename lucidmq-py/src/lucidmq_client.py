import socket
import cap_helper

class LucidmqClient:
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self.tcp_stream = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        print('Establishing TCP connection')
        self.tcp_stream.connect((host, port))
    
    def send_message_bytes(self, data: bytes) -> None:
        self.tcp_stream.sendall(data)
    
    def recieve_response(self) -> bytes:
        first_message = self.tcp_stream.recv(2)
        int_val = int.from_bytes(first_message, "little")
        second_message = self.tcp_stream.recv(int_val)
        return second_message

    def close_client() -> None:
        self.tcp_stream.close()

class Producer(LucidmqClient):
    def __init__(self, host, port):
        super().__init__(host, port)

    def produce(self, topic_name: str, key: bytes, value: bytes) -> dict:
        msg = cap_helper.produce_request(topic_name, key, value)
        self.send_message_bytes(msg)
        data = self.recieve_response()
        produce_response_obj = cap_helper.response_parser(data)
        return produce_response_obj.to_dict()
    
    def close(self) -> None:
        self.close_client()

class Consumer(LucidmqClient):
    def __init__(self, host: str, port: int, timeout: int):
        self.timeout = timeout
        super().__init__(host, port)

    def consume(self, topic_name: str, consumer_group: str) -> dict:
        msg = cap_helper.consume_request(topic_name, consumer_group, self.timeout)
        self.send_message_bytes(msg)
        data = self.recieve_response()
        cosumer_response_object = cap_helper.response_parser(data)
        return cosumer_response_object.to_dict()
    
    def close(self) -> None:
        self.close_client()

class TopicManager(LucidmqClient):
    def __init__(self, host, port):
        super().__init__(host, port)
    
    def create_topic(self, topic_name: str) -> dict:
        msg = cap_helper.topic_request_create(topic_name)
        self.send_message_bytes(msg)
        data = self.recieve_response()
        topic_response_object = cap_helper.response_parser(data)
        return topic_response_object.to_dict()
    
    def describe_topic(self, topic_name: str) -> dict:
        msg = cap_helper.topic_request_describe(topic_name)
        self.send_message_bytes(msg)
        data = self.recieve_response()
        topic_describe_object = cap_helper.response_parser(data)
        return topic_describe_object.to_dict()

    def delete_topic(self, topic_name: str) -> dict:
        msg = cap_helper.topic_request_delete(topic_name)
        self.send_message_bytes(msg)
        data = self.recieve_response()
        topic_delete_object = cap_helper.response_parser(data)
        return topic_delete_object.to_dict()
    
    def all_topic(self) -> dict:
        msg = cap_helper.topic_request_all()
        self.send_message_bytes(msg)
        data = self.recieve_response()
        topic_all_object = cap_helper.response_parser(data)
        return topic_all_object.to_dict()
        
    def close(self) -> None:
        self.close_client()