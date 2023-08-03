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
		return nil, err
	}
	envelope, err := protocol.ReadRootMessageEnvelope(cpaMsg)
	if err != nil {
		return nil, err
	}
	switch envelopeType := envelope.Which(); envelopeType {
	case protocol.MessageEnvelope_Which_topicResponse:
		topicResponse, err := envelope.TopicResponse()
		if err != nil {
			return nil, err
		}
		return topicResponseParser(topicResponse)
	case protocol.MessageEnvelope_Which_produceResponse:
		produceResponse, err := envelope.ProduceResponse()
		if err != nil {
			return nil, err
		}
		success := produceResponse.Success()
		topicName, err := produceResponse.TopicName()
		if err != nil {
			return nil, err
		}
		offset := produceResponse.Offset()
		produceResponseType := ProduceResponse{
			Success:   success,
			TopicName: topicName,
			Offset:    offset,
		}
		return produceResponseType, nil
	case protocol.MessageEnvelope_Which_consumeResponse:
		consumeResponse, err := envelope.ConsumeResponse()
		if err != nil {
			return nil, err
		}
		return parseConsumeResponse(consumeResponse)
	default:
		fmt.Println(envelopeType)
		fmt.Println("Message not valid")
		return i, errors.New("message is invalid")
	}
}

func topicResponseParser(topicResponse protocol.TopicResponse) (interface{}, error) {
	switch topicResponsetype := topicResponse.Which(); topicResponsetype {
	case protocol.TopicResponse_Which_all:
		success := topicResponse.Success()
		topicResponseAll, err := topicResponse.All()
		if err != nil {
			return nil, err
		}
		var topicsList []TopicList
		for i := 0; i < topicResponseAll.Len(); i++ {
			topic := topicResponseAll.At(i)
			topicName, err := topic.TopicName()
			if err != nil {
				return nil, err
			}
			consumerGroups, err := topic.ConsumerGroups()
			if err != nil {
				return nil, err
			}
			var cgs []string
			for j := 0; j < consumerGroups.Len(); j++ {
				consumerGroup, err := consumerGroups.At(j)
				if err != nil {
					return nil, err
				}
				cgs = append(cgs, consumerGroup)
			}
			topicsList = append(topicsList, TopicList{
				TopicName:      topicName,
				ConsumerGroups: cgs,
			})
		}
		return TopicAllResponse{
			Success:   success,
			TopicList: topicsList,
		}, nil
	case protocol.TopicResponse_Which_create:
		topicName, err := topicResponse.TopicName()
		if err != nil {
			return nil, err
		}
		success := topicResponse.Success()
		topicCreateResponse := TopicCreateResponse{
			Success:   success,
			TopicName: topicName,
		}
		return topicCreateResponse, nil
	case protocol.TopicResponse_Which_delete:
		topicName, err := topicResponse.TopicName()
		if err != nil {
			return nil, err
		}
		success := topicResponse.Success()
		topicDeleteResponse := TopicDeleteResponse{
			Success:   success,
			TopicName: topicName,
		}
		return topicDeleteResponse, nil
	case protocol.TopicResponse_Which_describe:
		topicName, err := topicResponse.TopicName()
		if err != nil {
			return nil, err
		}
		success := topicResponse.Success()
		topicResponseDescribe := topicResponse.Describe()
		maxSegmentBytes := topicResponseDescribe.MaxSegmentBytes()
		maxRetentionBytes := topicResponseDescribe.MaxRetentionBytes()
		consumerGroups, err := topicResponseDescribe.ConsumerGroups()
		if err != nil {
			return nil, err
		}
		var cgs []string
		for i := 0; i < consumerGroups.Len(); i++ {
			cg, err := consumerGroups.At(i)
			if err != nil {
				return nil, err
			}
			cgs = append(cgs, cg)
		}
		topicDescribeResponse := TopicDescribeResponse{
			Success:           success,
			TopicName:         topicName,
			MaxSegmentBytes:   maxSegmentBytes,
			MaxRetentionBytes: maxRetentionBytes,
			ConsumerGroups:    cgs,
		}
		return topicDescribeResponse, nil
	default:
		return nil, errors.New("topic response type is invalid")
	}
}

func parseConsumeResponse(consumeResponse protocol.ConsumeResponse) (interface{}, error) {
	success := consumeResponse.Success()
	topicName, err := consumeResponse.TopicName()
	if err != nil {
		return nil, err
	}
	messages, err := consumeResponse.Messages()
	if err != nil {
		return nil, err
	}
	var msgs []Message
	for i := 0; i < messages.Len(); i++ {
		message := messages.At(i)
		key, err := message.Key()
		if err != nil {
			return nil, err
		}
		value, err := message.Value()
		if err != nil {
			return nil, err
		}
		ts := message.Timestamp()
		msg := Message{
			Timestamp: ts,
			Key:       key,
			Value:     value,
		}
		msgs = append(msgs, msg)
	}

	return ConsumeResponse{
		Success:   success,
		TopicName: topicName,
		Messages:  msgs,
	}, nil
}
