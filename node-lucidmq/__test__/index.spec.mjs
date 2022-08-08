import test from 'ava'

import { Message } from '../index.js'

// test('Message from thing', (t) => {
//   // > const native = require('./index')
//   const buff = Buffer.from('hello');
//   const mess = new Message(buff, buff);
//   console.log(mess.serializeMessage())
//   //t.is(sum(1, 2), 3)
// })

// import { Message, LucidMQ } from './index.js'

// var maxSegmentSize = BigInt(1000n);
// var maxTopicSize = BigInt(100000n);
// const lucidmq = new LucidMQ("../xtestlog", maxSegmentSize, maxTopicSize);


// const buff = Buffer.from('hello');
// const mess = new Message(buff, buff);
// const producer = lucidmq.newProducer("topic1")

// producer.produceMessage(mess);

// const consumer = lucidmq.newConsumer("topic1", "cg1")
// let messages = consumer.poll(maxSegmentSize);
// for (const message of messages) {
//     console.log(message.getKey().toString())
// }

test('Message from thing', (t) => {
  // > const native = require('./index')
  const buff = Buffer.from('hello');
  const mess = new Message(buff, buff);
  console.log(mess.serializeMessage())
  //t.is(sum(1, 2), 3)
})