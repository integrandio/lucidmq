@0xfea86175937c8d2f;

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