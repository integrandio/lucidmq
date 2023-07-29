package lucidmq

import (
	"encoding/binary"
	"errors"
	"fmt"
	"protocol"
	"time"

	"capnproto.org/go/capnp/v3"
)

// Cap N Proto Helper functions
func topic_request_describe(topicName string) ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest, err := protocol.NewTopicRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest.SetDescribe()
	err = topicRequest.SetTopicName(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = envelope.SetTopicRequest(topicRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func topic_request_create(topicName string) ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest, err := protocol.NewTopicRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest.SetCreate()
	err = topicRequest.SetTopicName(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = envelope.SetTopicRequest(topicRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func topic_request_delete(topicName string) ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest, err := protocol.NewTopicRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest.SetDelete()
	err = topicRequest.SetTopicName(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = envelope.SetTopicRequest(topicRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func topic_request_all() ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest, err := protocol.NewTopicRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	topicRequest.SetAll()
	err = topicRequest.SetTopicName("placeholder")
	if err != nil {
		return []byte{}, err
	}
	err = envelope.SetTopicRequest(topicRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func produce_request(topicName string, key []byte, value []byte) ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	produceRequest, err := protocol.NewProduceRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	messageList, err := protocol.NewMessage_List(seg, 1)
	if err != nil {
		return []byte{}, err
	}

	message := messageList.At(0)
	err = message.SetKey(key)
	if err != nil {
		return []byte{}, err
	}
	err = message.SetValue(value)
	if err != nil {
		return []byte{}, err
	}
	now_milli := time.Now().UnixMilli()
	message.SetTimestamp(uint64(now_milli))

	produceRequest.SetMessages(messageList)
	err = produceRequest.SetTopicName(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = envelope.SetProduceRequest(produceRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func consume_request(topicName string, consumerGroup string, timeout uint64) ([]byte, error) {
	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
	if err != nil {
		return []byte{}, err
	}
	envelope, err := protocol.NewRootMessageEnvelope(seg)
	if err != nil {
		return []byte{}, err
	}
	consumeRequest, err := protocol.NewConsumeRequest(seg)
	if err != nil {
		return []byte{}, err
	}
	err = consumeRequest.SetTopicName(topicName)
	if err != nil {
		return []byte{}, err
	}
	err = consumeRequest.SetConsumerGroup(consumerGroup)
	if err != nil {
		return []byte{}, err
	}
	consumeRequest.SetTimout(timeout)

	err = envelope.SetConsumeRequest(consumeRequest)
	if err != nil {
		return []byte{}, err
	}
	b, err := msg.MarshalPacked()
	if err != nil {
		return []byte{}, err
	}
	framedMessageBytes := createMessageFrame(b)
	return framedMessageBytes, nil
}

func createMessageFrame(msg []byte) []byte {
	fullMsg := make([]byte, 2)
	binary.LittleEndian.PutUint16(fullMsg, uint16(len(msg)))
	fullMsg = append(fullMsg, msg...)
	return fullMsg
}

func responseParser(msg []byte) (interface{}, error) {
	var i interface{}
	cpaMsg, err := capnp.UnmarshalPacked(msg)
	if err != nil {
		panic(err)
	}
	envelope, err := protocol.ReadRootMessageEnvelope(cpaMsg)
	if err != nil {
		panic(err)
	}
	switch envelopeType := envelope.Which(); envelopeType {
	case protocol.MessageEnvelope_Which_topicResponse:
		topicResponse, err := envelope.TopicResponse()
		if err != nil {
			panic(err)
		}
		topicName, err := topicResponse.TopicName()
		if err != nil {
			panic(err)
		}
		success := topicResponse.Success()
		fmt.Println(topicName)
		fmt.Println(success)
		return i, nil
	case protocol.MessageEnvelope_Which_produceResponse:
		produceResponse, err := envelope.ProduceResponse()
		if err != nil {
			panic(err)
		}
		success := produceResponse.Success()
		topicName, err := produceResponse.TopicName()
		if err != nil {
			panic(err)
		}
		offset := produceResponse.Offset()
		produceResponseType := ProduceResponse{
			success:   success,
			topicName: topicName,
			offset:    offset,
		}
		return produceResponseType, nil
	case protocol.MessageEnvelope_Which_consumeResponse:
		consumeResponse, err := envelope.ConsumeResponse()
		if err != nil {
			panic(err)
		}
		success := consumeResponse.Success()
		topicName, err := consumeResponse.TopicName()
		if err != nil {
			panic(err)
		}
		messages, err := consumeResponse.Messages()
		if err != nil {
			panic(err)
		}
		fmt.Println(success)
		fmt.Println(topicName)
		fmt.Println(messages)
		return i, nil
	default:
		fmt.Println(envelopeType)
		fmt.Println("Message not valid")
		return i, errors.New("message is invalid")
	}
}
