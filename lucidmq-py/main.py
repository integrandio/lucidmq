
from src.lucidmq_client import TopicManager

HOST = "127.0.0.1" # The server's hostname or IP address
PORT = 6969 # The port used by the server

def main():
    print("Starting")
    topic_name = "testTopic"
    topic_manager = TopicManager(HOST, PORT)
    res = topic_manager.create_topic(topic_name)
    print(res)
    res = topic_manager.all_topic()
    print(res)
    
if __name__ == "__main__":
    main()

