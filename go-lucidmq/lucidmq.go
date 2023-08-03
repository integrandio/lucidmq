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

func (topicManager *TopicManager) CreateTopic(topicName string) (TopicCreateResponse, error) {
	var topicResponse TopicCreateResponse
	bytes, err := topic_request_create(topicName)
	if err != nil {
		return topicResponse, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return topicResponse, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return topicResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return topicResponse, err
	}

	topicResponse = responseMessage.(TopicCreateResponse)

	return topicResponse, nil
}

func (topicManager *TopicManager) DescribeTopic(topicName string) (TopicDescribeResponse, error) {
	var topicResponse TopicDescribeResponse
	bytes, err := topic_request_describe(topicName)
	if err != nil {
		return topicResponse, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return topicResponse, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return topicResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return topicResponse, err
	}

	topicResponse = responseMessage.(TopicDescribeResponse)

	return topicResponse, nil
}

func (topicManager *TopicManager) DeleteTopic(topicName string) (TopicDeleteResponse, error) {
	var topicResponse TopicDeleteResponse
	bytes, err := topic_request_delete(topicName)
	if err != nil {
		return topicResponse, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return topicResponse, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return topicResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return topicResponse, err
	}

	topicResponse = responseMessage.(TopicDeleteResponse)

	return topicResponse, nil
}

func (topicManager *TopicManager) AllTopics() (TopicAllResponse, error) {
	var topicResponse TopicAllResponse
	bytes, err := topic_request_all()
	if err != nil {
		return topicResponse, err
	}
	err = topicManager.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return topicResponse, err
	}

	responseBytes, err := topicManager.LucidmqClient.RecieveResponse()
	if err != nil {
		return topicResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return topicResponse, err
	}

	topicResponse = responseMessage.(TopicAllResponse)

	return topicResponse, nil
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

func (producer *Producer) Produce(topicName string, key []byte, value []byte) (ProduceResponse, error) {
	var produceResponse ProduceResponse
	bytes, err := produce_request(topicName, key, value)
	if err != nil {
		return produceResponse, err
	}
	err = producer.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return produceResponse, err
	}
	responseBytes, err := producer.LucidmqClient.RecieveResponse()
	if err != nil {
		return produceResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return produceResponse, err
	}

	produceResponse = responseMessage.(ProduceResponse)

	return produceResponse, nil
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

func (consumer *Consumer) Consume(topicName string, consumerGroup string) (ConsumeResponse, error) {
	var consumeResponse ConsumeResponse
	bytes, err := consume_request(topicName, consumerGroup, consumer.timeout)
	if err != nil {
		return consumeResponse, err
	}
	err = consumer.LucidmqClient.SendMessageBytes(bytes)
	if err != nil {
		return consumeResponse, err
	}
	responseBytes, err := consumer.LucidmqClient.RecieveResponse()
	if err != nil {
		return consumeResponse, err
	}

	responseMessage, err := responseParser(responseBytes)
	if err != nil {
		return consumeResponse, err
	}

	consumeResponse = responseMessage.(ConsumeResponse)
	return consumeResponse, nil
}
