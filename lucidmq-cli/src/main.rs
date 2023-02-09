mod client;
use clap::{arg, Command};
//use env_logger::Builder;
//use log::LevelFilter;

// fn create_topic(topic: String) {
//     let base_dir = String::from("../test_log");
//     let mut lucidmq = LucidMQ::new(base_dir, 1000, 5000);

//     lucidmq.new_topic(topic);
// }

// fn run_producer(topic: String) {
//     let base_dir = String::from("../test_log");
//     let mut lucidmq = LucidMQ::new(base_dir, 1000, 5000);

//     let mut producer = lucidmq.new_producer(topic);
//     let second = Duration::from_millis(1000);

//     for i in 0..100 {
//         let key = "producer1".to_string();
//         let key_bytes = key.as_bytes();
//         let value = format!("
//         {{
//             \"id\": {},
//             \"price\": {},
//             \"description\": \"my description\"
//         }}", i, i*100);
//         let value_bytes = value.as_bytes();
//         let message = Message::new(key_bytes, value_bytes, None);
//         producer.produce_message(message);
//         thread::sleep(second);
//     }
// }

// fn run_consumer(topic: String, consumer_group: String) {
//     let base_dir = String::from("../test_log");
//     let mut lucidmq = LucidMQ::new(base_dir, 1000, 5000);
//     let mut consumer = lucidmq.new_consumer(topic, consumer_group);
//     loop {
//         let records = consumer.poll(2000);
//         for record in records {
//             println!("--------------------------");
//             println!("{}", str::from_utf8(&record.key).unwrap());
//             println!("{}", str::from_utf8(&record.value).unwrap());
//             println!("{}", record.timestamp);
//         }
//     }
// }

/*
What commands do we want to expose to the cli?
Create lucidmq?
-> View topics
-> View consumergroups
Produce message?
Consume messages?
 */

fn cli() -> Command<'static> {
    Command::new("LucidMQ")
        .about("A tool to interact with your LucidMQ instance")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("topic")
                .about("Create a new topic")
                .arg(arg!(<TOPIC> "The topic name you want to create"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("produce")
                .about("produce messages")
                .arg(arg!(<TOPIC> "The topic you want to produce to"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("consume")
                .about("consume messages")
                .arg(arg!(<TOPIC> "The topic you want to consume from"))
                .arg_required_else_help(true)
                .arg(arg!(<CONSUMER_GROUP> "The consumer group you want to use"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5000".parse().unwrap();
    let res = client::run_client(addr).await;
    res.expect("NO ERRORS PLEASSE")
    // Builder::new().filter_level(LevelFilter::max()).init();
    // let matches = cli().get_matches();

    // match matches.subcommand() {
    //     Some(("topic", sub_matches)) => {
    //         let topic_name = sub_matches.get_one::<String>("TOPIC").expect("required");
    //         println!("Creating topic {}", topic_name);
    //         //create_topic(topic_name.to_string());
    //     }
    //     Some(("produce", sub_matches)) => {
    //         let topic_name = sub_matches.get_one::<String>("TOPIC").expect("required");
    //         println!("producing to {}", topic_name);
    //         //run_producer(topic_name.to_string());
    //     }
    //     Some(("consume", sub_matches)) => {
    //         let topic_name = sub_matches.get_one::<String>("TOPIC").expect("required");
    //         let consumer_group_name = sub_matches
    //             .get_one::<String>("CONSUMER_GROUP")
    //             .expect("required");
    //         println!(
    //             "Consuming from {}  with {}",
    //             topic_name, consumer_group_name
    //         );
    //         //run_consumer(topic_name.to_string(), consumer_group_name.to_string());
    //     }
    //     _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    // }

    // Continued program logic goes here...
}
