mod client;
use clap::{arg, Command};
use env_logger::Builder;
use log::LevelFilter;
mod request_builder;
pub mod lucid_schema_capnp;
mod cap_n_proto_helper;
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
            Command::new("connect")
                .about("Connect to a LucidMQ broker instance")
                .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
                .arg(arg!(<PORT> "The port where your LucidMQ is"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();    
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("connect", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");

            let connection_string = format!("{}:{}", address, port).parse().unwrap();
            let res = client::run_client(connection_string).await;
            res.expect("Unable to connect to server");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
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
