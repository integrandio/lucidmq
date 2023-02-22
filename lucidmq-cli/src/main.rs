mod client;
use clap::{arg, Command};
mod request_builder;
mod response_parser;
pub mod lucid_schema_capnp;
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
