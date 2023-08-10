package integration

import (
	"testing"

	"lucidmq.com/lucidmq/go-lucidmq"
)

func init() {
	getSetGlobalVariables()
}

// Topic tests
func TestCreateTopic(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	topicResponse, err := topicManger.CreateTopic(topicName)
	ok(t, err)
	assert(t, topicResponse.Success == true,
		"Expected topic response success %v, but got %v",
		false,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)

	topicManger.DeleteTopic(topicName)
}

func TestCreateTopicAlreadyExists(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	// Set up the topic
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	topicResponse, err := topicManger.CreateTopic(topicName)
	ok(t, err)

	assert(t, topicResponse.Success == false,
		"Expected topic response success %v, but got %v",
		true,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)

	topicManger.DeleteTopic(topicName)
}

func TestDescribeTopic(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)
	//Set up the topic
	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	topicResponse, err := topicManger.DescribeTopic(topicName)
	ok(t, err)

	assert(t, topicResponse.Success == true,
		"Expected topic response success %v, but got %v",
		false,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)

	topicManger.DeleteTopic(topicName)
}

func TestDescribeTopicDNE(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)

	topicResponse, err := topicManger.DescribeTopic(topicName)
	ok(t, err)

	assert(t, topicResponse.Success == false,
		"Expected topic response success %v, but got %v",
		true,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)

	topicManger.DeleteTopic(topicName)
}

func TestDeleteTopic(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)

	_, err = topicManger.CreateTopic(topicName)
	ok(t, err)

	topicResponse, err := topicManger.DeleteTopic(topicName)
	ok(t, err)

	assert(t, topicResponse.Success == true,
		"Expected topic response success %v, but got %v",
		false,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)
	topicManger.DeleteTopic(topicName)
}

func TestDeleteTopicDNE(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	ok(t, err)
	topicName := generateRandomString(10)

	topicResponse, err := topicManger.DeleteTopic(topicName)
	ok(t, err)

	assert(t, topicResponse.Success == false,
		"Expected topic response success %v, but got %v",
		true,
		topicResponse.Success)

	assert(t, topicResponse.TopicName == topicName,
		"Expected topic response name %s, but got %s",
		topicName,
		topicResponse.TopicName)
	topicManger.DeleteTopic(topicName)
}
