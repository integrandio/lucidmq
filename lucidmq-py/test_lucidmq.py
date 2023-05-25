from lucidmq_client import Producer, Consumer, TopicManager

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 5000  # The port used by the server

class TestTopics:
    def test_topic_create(self):
        topic_name = "testTopic"
        topic_manager = TopicManager(HOST, PORT)
        # Create a topic
        topic_create_result = topic_manager.create_topic(topic_name)
        assert topic_create_result['success'] == True
        assert topic_create_result['topicName'] == topic_name
        # Delete the topic to clean up
        topic_delete_result = topic_manager.delete_topic(topic_name)

    def test_create_topic_already_exists(self):
        topic_name = "testTopic0"
        topic_manager = TopicManager(HOST, PORT)
        topic_create_result = topic_manager.create_topic(topic_name)
        topic_create_result = topic_manager.create_topic(topic_name)
        assert topic_create_result['success'] == False
        assert topic_create_result['topicName'] == topic_name
        # Delete the topic to clean up
        topic_delete_result = topic_manager.delete_topic(topic_name)

    def test_topic_describe(self):
        topic_name = "testTopic"
        topic_manager = TopicManager(HOST, PORT)
        # Create a topic
        topic_create_result = topic_manager.create_topic(topic_name)
        topic_describe_result = topic_manager.describe_topic(topic_name)
        assert topic_describe_result['success'] == True
        assert topic_describe_result['topicName'] == topic_name
        # Delete the topic to clean up
        topic_delete_result = topic_manager.delete_topic(topic_name)


    def test_describe_topic_dne(self):
        topic_name = "topicDoesNotExist"
        topic_manager = TopicManager(HOST, PORT)
        topic_describe_result = topic_manager.describe_topic(topic_name)
        assert topic_describe_result['success'] == False
        assert topic_describe_result['topicName'] == topic_name
    

    def test_delete_topic_dne(self):
        topic_name = "topicDoesNotExist"
        topic_manager = TopicManager(HOST, PORT)
        topic_delete_result = topic_manager.delete_topic(topic_name)
        assert topic_delete_result['success'] == False
        assert topic_delete_result['topicName'] == topic_name

