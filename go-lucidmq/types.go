package lucidmq

type TopicCreateResponse struct {
	success   bool
	topicName string
}

type TopicDeleteResponse struct {
	success   bool
	topicName string
}

// type TopicDescribeResponse struct {
// 	success           bool
// 	topicName         string
// 	maxSegmentBytes   uint64
// 	maxRetentionBytes uint64
// 	consumerGroups    []string
// }

type ProduceResponse struct {
	success   bool
	topicName string
	offset    uint64
}
