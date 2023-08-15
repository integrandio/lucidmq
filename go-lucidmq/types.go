package lucidmq

type TopicCreateResponse struct {
	Success   bool
	TopicName string
}

type TopicDeleteResponse struct {
	Success   bool
	TopicName string
}

type TopicDescribeResponse struct {
	Success           bool
	TopicName         string
	MaxSegmentBytes   uint64
	MaxRetentionBytes uint64
	ConsumerGroups    []string
}

type TopicAllResponse struct {
	Success   bool
	TopicList []TopicList
}

type TopicList struct {
	TopicName      string
	ConsumerGroups []string
}

type ProduceResponse struct {
	Success   bool
	TopicName string
	Offset    uint64
}

type ConsumeResponse struct {
	Success   bool
	TopicName string
	Messages  []Message
}

type Message struct {
	Timestamp uint64
	Key       []byte
	Value     []byte
}

type InvalidResponse struct {
	ErrorMessage string
}
