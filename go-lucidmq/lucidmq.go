package lucidmq

import (
	"encoding/binary"
	"fmt"
	"net"
)

type LucidmqClient struct {
	host       string
	port       int
	connection net.Conn
}

func NewLucidmqClient(host string, port int) (*LucidmqClient, error) {
	address := fmt.Sprintf("%s:%d", host, port)
	conn, err := net.Dial("tcp", address)
	if err != nil {
		return nil, err
	}

	lucidmqClient := &LucidmqClient{
		host:       host,
		port:       port,
		connection: conn,
	}
	return lucidmqClient, nil
}

func (lucidmqClient *LucidmqClient) SendMessageBytes(data []byte) error {
	_, err := lucidmqClient.connection.Write(data)
	return err
}

func (lucidmqClient *LucidmqClient) RecieveResponse() ([]byte, error) {
	sizeBuffer := make([]byte, 2)
	_, err := lucidmqClient.connection.Read(sizeBuffer)
	if err != nil {
		return []byte{}, err
	}
	size := binary.LittleEndian.Uint16(sizeBuffer)

	messageBuffer := make([]byte, size)

	_, err = lucidmqClient.connection.Read(messageBuffer)
	if err != nil {
		return []byte{}, err
	}
	return messageBuffer, nil
}

func (lucidmqClient *LucidmqClient) CloseClient() error {
	err := lucidmqClient.connection.Close()
	return err
}

type TopicManager struct {
	LucidmqClient
}

func NewTopicManger(host string, port int) (*TopicManager, error) {
	lmc, err := NewLucidmqClient(host, port)
	if err != nil {
		return nil, err
	}
	topicManger := &TopicManager{
		LucidmqClient: *lmc,
	}

	return topicManger, nil
}

func (topicManager *TopicManager) CreateTopic(topicName string) ([]byte, error) {
	bytes, err := topic_request_create(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

func (topicManager *TopicManager) DescribeTopic(topicName string) ([]byte, error) {
	bytes, err := topic_request_describe(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

func (topicManager *TopicManager) DeleteTopic(topicName string) ([]byte, error) {
	bytes, err := topic_request_delete(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

func (topicManager *TopicManager) AllTopics() ([]byte, error) {
	bytes, err := topic_request_all()
	if err != nil {
		return []byte{}, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

type Producer struct {
	LucidmqClient
}

func NewProducer(host string, port int) (*Producer, error) {
	lmc, err := NewLucidmqClient(host, port)
	if err != nil {
		return nil, err
	}
	producer := &Producer{
		LucidmqClient: *lmc,
	}

	return producer, nil
}

func (producer *Producer) Produce(topicName string, key []byte, value []byte) ([]byte, error) {
	bytes, err := produce_request(topicName, key, value)
	if err != nil {
		return []byte{}, err
	}
	err = producer.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}
	responseBytes, err := producer.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

type Consumer struct {
	timeout uint64
	LucidmqClient
}

func NewConsumer(host string, port int, timeout uint64) (*Consumer, error) {
	lmc, err := NewLucidmqClient(host, port)
	if err != nil {
		return nil, err
	}
	consumer := &Consumer{
		timeout:       timeout,
		LucidmqClient: *lmc,
	}

	return consumer, nil
}

func (consumer *Consumer) Consume(topicName string, consumerGroup string) ([]byte, error) {
	bytes, err := consume_request(topicName, consumerGroup, consumer.timeout)
	if err != nil {
		return []byte{}, err
	}
	err = consumer.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return []byte{}, err
	}
	responseBytes, err := consumer.LucidmqClient.RecieveResponse()
	if err != nil {
		return []byte{}, err
	}

	return responseBytes, nil
}

func main() {
	host := "localhost"
	port := 6969

	// producer, err := NewProducer(host, port)
	// if err != nil {
	// 	panic(err)
	// }

	// responseBytes, err := producer.produce("topic1", []byte("key"), []byte("value"))
	// if err != nil {
	// 	panic(err)
	// }

	consumer, err := NewConsumer(host, port, 1)
	if err != nil {
		panic(err)
	}

	responseBytes, err := consumer.Consume("topic1", "cg3")
	if err != nil {
		panic(err)
	}
	fmt.Println(responseBytes)
	thang, err := responseParser(responseBytes)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%v", thang)
}
