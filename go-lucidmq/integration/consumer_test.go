package integration

import (
	"fmt"
	"testing"

	"lucidmq.com/lucidmq/go-lucidmq"
)

// Consumer tests
func TestConsume1Message(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)
	key := []byte("myKey")
	value := []byte("myValue")
	_, err = producer.Produce(topicName, key, value)
	ok(t, err)

	consumer, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumeResponse, err := consumer.Consume(topicName, "cg1")
	ok(t, err)

	assert(t, consumeResponse.Success == true,
		"Expected consume response success %v, but got %v",
		false,
		consumeResponse.Success)

	assert(t, consumeResponse.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse.TopicName)

	consumeResponseMessageLength := len(consumeResponse.Messages)
	assert(t, consumeResponseMessageLength == 1,
		"Expected number of messages %d, but got %d",
		1,
		consumeResponseMessageLength)

	message := consumeResponse.Messages[0]

	equals(t, key, message.Key)
	equals(t, value, message.Value)

	topicManger.DeleteTopic(topicName)
}

func TestConsume10Messages(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)

	var keysSent [][]byte
	var valuesSent [][]byte

	for i := 0; i < 10; i++ {
		key := []byte(fmt.Sprintln("key", i))
		value := []byte(fmt.Sprintln("value", i))
		keysSent = append(keysSent, key)
		valuesSent = append(valuesSent, value)

		_, err = producer.Produce(topicName, key, value)
		ok(t, err)
	}

	consumer, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumeResponse, err := consumer.Consume(topicName, "cg1")
	ok(t, err)

	assert(t, consumeResponse.Success == true,
		"Expected consume response success %v, but got %v",
		false,
		consumeResponse.Success)

	assert(t, consumeResponse.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse.TopicName)

	consumeResponseMessageLength := len(consumeResponse.Messages)
	assert(t, consumeResponseMessageLength == 10,
		"Expected number of messages %d, but got %d",
		10,
		consumeResponseMessageLength)

	for j := 0; j < 10; j++ {
		message := consumeResponse.Messages[j]
		equals(t, keysSent[j], message.Key)
		equals(t, valuesSent[j], message.Value)
	}

	topicManger.DeleteTopic(topicName)
}

func TestConsumerNoMessage(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	consumer, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumeResponse, err := consumer.Consume(topicName, "cg1")
	ok(t, err)

	assert(t, consumeResponse.Success == false,
		"Expected consume response success %v, but got %v",
		true,
		consumeResponse.Success)

	assert(t, consumeResponse.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse.TopicName)

	consumeResponseMessageLength := len(consumeResponse.Messages)
	assert(t, consumeResponseMessageLength == 0,
		"Expected number of messages %d, but got %d",
		0,
		consumeResponseMessageLength)

	topicManger.DeleteTopic(topicName)
}

func TestConsumerTopicDNE(t *testing.T) {
	topicName := generateRandomString(10)

	consumer, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumeResponse, err := consumer.Consume(topicName, "cg1")
	ok(t, err)

	assert(t, consumeResponse.Success == false,
		"Expected consume response success %v, but got %v",
		true,
		consumeResponse.Success)

	assert(t, consumeResponse.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse.TopicName)

	consumeResponseMessageLength := len(consumeResponse.Messages)
	assert(t, consumeResponseMessageLength == 0,
		"Expected number of messages %d, but got %d",
		0,
		consumeResponseMessageLength)
}

func TestConsumerMessageAlreadyConsumed(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	consumerGroup := "cg1"
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)
	key := []byte("myKey")
	value := []byte("myValue")
	_, err = producer.Produce(topicName, key, value)
	ok(t, err)

	consumer0, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumer1, err := lucidmq.NewConsumer(HOST, PORT, 100)
	ok(t, err)

	consumeResponse0, err := consumer0.Consume(topicName, consumerGroup)
	ok(t, err)

	assert(t, consumeResponse0.Success == true,
		"Expected consume response success %v, but got %v",
		false,
		consumeResponse0.Success)

	assert(t, consumeResponse0.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse0.TopicName)

	consumeResponseMessageLength := len(consumeResponse0.Messages)
	assert(t, consumeResponseMessageLength == 1,
		"Expected number of messages %d, but got %d",
		1,
		consumeResponseMessageLength)

	message := consumeResponse0.Messages[0]

	equals(t, key, message.Key)
	equals(t, value, message.Value)

	consumeResponse1, err := consumer1.Consume(topicName, consumerGroup)
	ok(t, err)

	assert(t, consumeResponse1.Success == false,
		"Expected consume response success %v, but got %v",
		true,
		consumeResponse1.Success)

	assert(t, consumeResponse1.TopicName == topicName,
		"Expected consume response name %s, but got %s",
		topicName,
		consumeResponse1.TopicName)

	consumeResponseMessageLength0 := len(consumeResponse1.Messages)
	assert(t, consumeResponseMessageLength0 == 0,
		"Expected number of messages %d, but got %d",
		0,
		consumeResponseMessageLength0)

	topicManger.DeleteTopic(topicName)
}
