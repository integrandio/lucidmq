using Go = import "/go.capnp";
@0xa0042efeeed7bb94;
$Go.package("protocol");
$Go.import ("lucidmq/protocol");

# --- Message Envelope ----
struct MessageEnvelope {
  union {
    topicRequest @0 :TopicRequest;
    topicResponse @1 :TopicResponse;
    produceRequest @2 :ProduceRequest;
    produceResponse @3 :ProduceResponse;
    consumeRequest @4 :ConsumeRequest;
    consumeResponse @5 :ConsumeResponse;
  }
}

# ----- Topic Messages --------
struct TopicRequest {
  topicName @0 :Text;
  union {
    describe @1 :Void;
    create @2 :Void;
    delete @3 :Void;
    all @4 :Void;
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
    all @7 :List(TopicsList);
  }
}

struct TopicsList {
  topicName @0 :Text;
  consumerGroups @1 :List(Text);
}

struct ProduceRequest {
  topicName @0 :Text;
  messages @1 :List(Message);
}

struct ProduceResponse {
  success @0 :Bool;
  topicName @1 :Text;
  offset @2 :UInt64;
}

struct ConsumeRequest {
  topicName @0 :Text;
  consumerGroup @1 :Text;
  timout @2 :UInt64;
}

struct ConsumeResponse {
  success @0 :Bool;
  topicName @1 :Text;
  messages @2 :List(Message);
}

struct Message {
  timestamp @0 :UInt64;
  key @1 :Data;
  value @2 :Data;
}