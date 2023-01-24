@0xe788bf9343d44e3f;

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