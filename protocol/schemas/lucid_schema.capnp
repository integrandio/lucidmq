@0xa0042efeeed7bb94;

# ----- Topic Messages --------

struct TopicRequest {
  topicName @0 :Text;
  union {
    describe @1 :Void;
    create @2 :Void;
    delete @3 :Void;
  }
}

struct TopicResponse {
  topicName @0 :Text;
  success @1 :Bool;
  union {
    describe :group {
        maxSegmentBytes @2 :UInt64;
        maxRetentionBytes @3 :UInt64;
        consumerGroups @4 :List(Text);
    }
    create @5 :Void;
    delete @6 :Void;
  }
}

struct ProduceRequest {
  topicName @0 :Text;
  messages @1 :List(Message);
}

struct ProduceResponse {
  topicName @0 :Text;
  offset @1 :UInt64;
}

struct ConsumeRequest {
  topicName @0 :Text;
  consumerGroup @1 :Text;
  timout @2 :UInt64;
}

struct ConsumeResponse {
  success @0 :Bool;
  messages @1 :List(Message);
}

struct Message {
  timestamp @0 :UInt64;
  key @1 :Data;
  value @2 :Data;
}