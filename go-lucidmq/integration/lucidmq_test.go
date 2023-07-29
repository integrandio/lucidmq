package integration

import (
	"fmt"
	"testing"

	"lucidmq.com/lucidmq/go-lucidmq"
)

const HOST = "localhost"
const PORT = 6969

// Topic tests
func TestNewTopic(t *testing.T) {
	topicManger, err := lucidmq.NewTopicManger(HOST, PORT)
	if err != nil {
		panic(err)
	}

	responseBytes, err := topicManger.CreateTopic("topic1")
	if err != nil {
		panic(err)
	}
	fmt.Println(responseBytes)
}

// Producer Tests
func TestNewProducer(t *testing.T) {
	producer, err := lucidmq.NewProducer(HOST, PORT)
	if err != nil {
		panic(err)
	}

	responseBytes, err := producer.Produce("topic1", []byte("key"), []byte("value"))
	if err != nil {
		panic(err)
	}

	fmt.Println(responseBytes)

}

func TestHelloWorld(t *testing.T) {
	fmt.Println("hello world")
}
