package main

// func topic_request_create(topicName string) []byte {
// 	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
// 	if err != nil {
// 		panic(err)
// 	}
// 	envelope, err := protocol.NewRootMessageEnvelope(seg)
// 	if err != nil {
// 		panic(err)
// 	}
// 	topicRequest, err := protocol.NewTopicRequest(seg)
// 	if err != nil {
// 		panic(err)
// 	}
// 	topicRequest.SetCreate()
// 	err = topicRequest.SetTopicName(topicName)
// 	if err != nil {
// 		panic(err)
// 	}
// 	err = envelope.SetTopicRequest(topicRequest)
// 	if err != nil {
// 		panic(err)
// 	}
// 	b, err := msg.Marshal()
// 	if err != nil {
// 		panic(err)
// 	}
// 	return b
// }

// func topic_request_delete(topicName string) ([]byte, error) {
// 	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	envelope, err := protocol.NewRootMessageEnvelope(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest, err := protocol.NewTopicRequest(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest.SetDelete()
// 	err = topicRequest.SetTopicName(topicName)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	err = envelope.SetTopicRequest(topicRequest)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	b, err := msg.Marshal()
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	return b, nil
// }

// func topic_request_all() ([]byte, error) {
// 	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	envelope, err := protocol.NewRootMessageEnvelope(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest, err := protocol.NewTopicRequest(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest.SetAll()
// 	err = topicRequest.SetTopicName("placeholder")
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	err = envelope.SetTopicRequest(topicRequest)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	b, err := msg.Marshal()
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	return b, nil
// }

// func topic_request() ([]byte, error) {
// 	msg, seg, err := capnp.NewMessage(capnp.SingleSegment(nil))
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	envelope, err := protocol.NewRootMessageEnvelope(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest, err := protocol.NewProduceRequest(seg)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	topicRequest.SetAll()
// 	err = topicRequest.SetTopicName("placeholder")
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	err = envelope.SetTopicRequest(topicRequest)
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	b, err := msg.Marshal()
// 	if err != nil {
// 		return []byte{}, err
// 	}
// 	return b, nil
// }
