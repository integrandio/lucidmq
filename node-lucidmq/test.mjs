import { Message, LucidMQ } from './index.js'

var maxSegmentSize = BigInt(1000n);
var maxTopicSize = BigInt(100000n);
const lucidmq = new LucidMQ("../test_log", maxSegmentSize, maxTopicSize);

console.log("program started");

// const buff = Buffer.from('hello');
// const mess = new Message(buff, buff);
// const producer = lucidmq.newProducer("topic1")

// producer.produceMessage(mess);

const consumer = lucidmq.newConsumer("topic1", "cg2")
let messages = consumer.poll(maxSegmentSize);
for (const message of messages) {
    console.log(message.getKey().toString() + " : "+ message.getValue().toString());
}


// const props = Object.getOwnPropertyNames(mess);

// console.log(props);    // [ 'name', 'age' ]
// //console.log(mess.serializeMessage())

// // const buff2 = Buffer.from('gay');
// // const message = new Message(buff2, buff2);
// // console.log(message.serializeMessage())