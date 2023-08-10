package integration

import (
	"fmt"
	"testing"

	"lucidmq.com/lucidmq/go-lucidmq"
)

func init() {
	getSetGlobalVariables()
}

// Producer tests
func TestProduce1Message(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)
	produceResponse, err := producer.Produce(topicName, []byte("myKey"), []byte("myValue"))
	ok(t, err)

	assert(t, produceResponse.Success == true,
		"Expected produce response success %v, but got %v",
		false,
		produceResponse.Success)

	assert(t, produceResponse.TopicName == topicName,
		"Expected produce response name %s, but got %s",
		topicName,
		produceResponse.TopicName)

	assert(t, produceResponse.Offset == 0,
		"Expected produce response name %d, but got %d",
		0,
		produceResponse.Offset)

	topicManger.DeleteTopic(topicName)
}

func TestProduce10Messages(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)

	for i := 0; i < 10; i++ {
		key := fmt.Sprintln("key", i)
		value := fmt.Sprintln("value", i)
		produceResponse, err := producer.Produce(topicName, []byte(key), []byte(value))
		ok(t, err)

		assert(t, produceResponse.Success == true,
			"Expected produce response success %v, but got %v",
			false,
			produceResponse.Success)

		assert(t, produceResponse.TopicName == topicName,
			"Expected produce response name %s, but got %s",
			topicName,
			produceResponse.TopicName)

		assert(t, produceResponse.Offset == uint64(i),
			"Expected produce response offset %d, but got %d",
			i,
			produceResponse.Offset)
	}

	topicManger.DeleteTopic(topicName)
}

func TestProduce30LargeMessages(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up our topis
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)

	for i := 0; i < 10; i++ {
		key := fmt.Sprintln("key", i)
		value := fmt.Sprintln("myextreamlyverylargevalue", i)
		produceResponse, err := producer.Produce(topicName, []byte(key), []byte(value))
		ok(t, err)

		assert(t, produceResponse.Success == true,
			"Expected produce response success %v, but got %v",
			false,
			produceResponse.Success)

		assert(t, produceResponse.TopicName == topicName,
			"Expected produce response name %s, but got %s",
			topicName,
			produceResponse.TopicName)

		assert(t, produceResponse.Offset == uint64(i),
			"Expected produce response offset %d, but got %d",
			i,
			produceResponse.Offset)
	}

	topicManger.DeleteTopic(topicName)
}

func TestProduceTopicDNE(t *testing.T) {
	topicName := generateRandomString(10)

	producer, err := lucidmq.NewProducer(HOST, PORT)
	ok(t, err)

	produceResponse, err := producer.Produce(topicName, []byte("myKey"), []byte("myValue"))
	ok(t, err)

	assert(t, produceResponse.Success == false,
		"Expected produce response success %v, but got %v",
		true,
		produceResponse.Success)

	assert(t, produceResponse.TopicName == topicName,
		"Expected produce response name %s, but got %s",
		topicName,
		produceResponse.TopicName)

	assert(t, produceResponse.Offset == 0,
		"Expected produce response name %d, but got %d",
		0,
		produceResponse.Offset)

}
