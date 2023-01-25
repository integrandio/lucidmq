use capnp::message::Builder;
use capnp::message::ReaderOptions;
use capnp::serialize;
use crate::cnp_capnp;
use crate::topic_capnp;

pub fn create_prooducer_message() -> Vec<u8> {
    // Creating object
    let mut cap_builder = Builder::new_default();

    let mut message = cap_builder.init_root::<cnp_capnp::message::Builder>();
    message.set_key("key".as_bytes());
    message.set_value("value".as_bytes());
    message.set_timestamp(1);

    // Serializing object
    let data = serialize::write_message_to_words(&cap_builder);
    println!("{:?}", data);
    return data;

    // // Deserializing object
    // let reader = serialize::read_message(
    // data.as_slice(),
    // ReaderOptions::new()
    // ).unwrap();

    // let person = reader.get_root::<person_schema_capnp::produce::Reader>().unwrap();
    // let thing0 = person.get_messages().unwrap().get(0).get_timestamp();
    // let name = person.get_timeout();
    // println!("timeout: {:?}", thing0);
}