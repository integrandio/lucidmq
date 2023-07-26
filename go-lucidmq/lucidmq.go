package main

import (
	"encoding/binary"
	"fmt"
	"net"
	"protocol"

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
		panic(err)
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
	return b, nil
}

func main() {
	host := "localhost"
	port := 6969
	address := fmt.Sprintf("%s:%d", host, port)

	connection, err := net.Dial("tcp", address)
	if err != nil {
		panic(err)
	}

	bytes, err := topic_request_describe("topic1")
	if err != nil {
		panic(err)
	}

	framedMessageBytes := createMessageFrame(bytes)

	_, err = connection.Write(framedMessageBytes)
	if err != nil {
		panic(err)
	}

	messageSize, err := getSocketMessageSize(connection)
	if err != nil {
		panic(err)
	}

	fmt.Println(messageSize)
	messageBuffer := make([]byte, messageSize)

	_, err = connection.Read(messageBuffer)
	if err != nil {
		panic(err)
	}

	fmt.Println(messageBuffer)

	err = connection.Close()
	if err != nil {
		panic(err)
	}
}

func getSocketMessageSize(conn net.Conn) (uint16, error) {
	p := make([]byte, 2)
	_, err := conn.Read(p)
	if err != nil {
		return uint16(0), err
	}
	size := binary.LittleEndian.Uint16(p)
	return size, nil
}

func createMessageFrame(msg []byte) []byte {
	fullMsg := make([]byte, 2)
	binary.LittleEndian.PutUint16(fullMsg, uint16(len(msg)))
	fullMsg = append(fullMsg, msg...)
	return fullMsg
}
