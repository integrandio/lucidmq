package integration

import (
	"testing"

	"lucidmq.com/lucidmq/go-lucidmq"
)

func init() {
	getSetGlobalVariables()
}

// Producer tests
func TestInvalidMessage(t *testing.T) {
	lucidClient, err := lucidmq.NewLucidmqClient(HOST, PORT)
	ok(t, err)
	invalidData := []byte("trash")
	err = lucidClient.SendMessageBytes(invalidData)
	ok(t, err)
	responseBytes, err := lucidClient.RecieveResponse()
	ok(t, err)
	responseType, err := lucidmq.ResponseParser(responseBytes)
	ok(t, err)
	invalidResponse := responseType.(lucidmq.InvalidResponse)

	assert(t, invalidResponse.ErrorMessage == "invalid message sent",
		"Expected string in message: [%v], but got [%v]",
		"invalid message sent",
		invalidResponse.ErrorMessage)
}
