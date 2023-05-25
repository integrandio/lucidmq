mod tcp_client;
use std::io::Write;
use std::{thread, time};
use std::{net::SocketAddr};
use clap::{arg, Command};
use env_logger::Builder;
use log::{LevelFilter, info};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
mod request_builder;
pub mod lucid_schema_capnp;
mod cap_n_proto_helper;
use std::io::{self, BufRead};


fn base_cli() -> Command<'static> {
    Command::new("LucidMQ")
        .about("A tool to interact with your LucidMQ instance")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("connect")
                .about("Create an interactive connection to a LucidMQ broker instance")
                .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
                .arg(arg!(<PORT> "The port where your LucidMQ is"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("producer")
                .about("Create a producer instance that takes in piped arguements to produce.")
                .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
                .arg(arg!(<PORT> "The port where your LucidMQ is"))
                .arg(arg!(<TOPIC_NAME> "The topice where you want to produce messages to"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("consumer")
            .about("Create a consumer instance that allows you pipe out data.")
            .arg(arg!(<ADDRESS> "The address where your LucidMQ is"))
            .arg(arg!(<PORT> "The port where your LucidMQ is"))
            .arg(arg!(<TOPIC_NAME> "The topice where you want to consume from"))
            .arg(arg!(<CONSUMER_GROUP> "The consumer group you want to use"))
        )
}

fn respond(line: &str) -> Result<Vec<u8>, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = interactive_cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("produce", sub_matches)) => {
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");
            let msg = "value".as_bytes();
            return Ok(request_builder::new_produce_request(topic_name, msg));
        }
        Some(("consume", sub_matches)) => {
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");
            let consumer_group = sub_matches.get_one::<String>("CONSUMER_GROUP").expect("required");
            return Ok(request_builder::new_consume_message(topic_name, consumer_group));
        }
        Some(("topic", sub_matches)) => {
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");
            let operation_type = sub_matches.get_one::<String>("TYPE").expect("required");
            return Ok(request_builder::new_topic_request(topic_name, operation_type));
        }
        Some(("quit", _matches)) => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok("0".as_bytes().to_vec());
        }
        Some((name, _matches)) => unimplemented!("{}", name),
        None => unreachable!("subcommand required"),
    }

    //Ok("0".as_bytes().to_vec())
}

fn interactive_cli() -> Command<'static> {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("produce")
                .about("Get a response")
                .arg(arg!(<TOPIC_NAME> "The topic to produce to"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("consume")
                .arg(arg!(<TOPIC_NAME> "The topic to consume from"))
                .arg(arg!(<CONSUMER_GROUP> "The consumer group to use"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("topic")
                .arg(arg!(<TYPE> "The topic request message tye"))
                .arg(arg!(<TOPIC_NAME> "The topic to consume from"))
                .arg_required_else_help(true)
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}

fn readline() -> Result<String, String> {
    // Wait for client code to cleanup.
    thread::sleep(time::Duration::from_millis(500));
    write!(std::io::stdout(), "> ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn thinger(stdin_tx: UnboundedSender<Vec<u8>>) -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(msg) => {
                let msg_size = msg.len();
                stdin_tx.send(msg).expect("Unable to send message");
                //TODO: Change this to be more robust
                if msg_size == 1 {
                    // Wait for client code to cleanup.
                    thread::sleep(time::Duration::from_millis(500));
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }
    info!("Exiting...");
    Ok(())
}

fn stdin_processor(stdin_tx: UnboundedSender<Vec<u8>>, topic_name: &str) -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer)?;
        if buffer.len() == 0 {
            continue;
        }
        // Can these requests be batched?
        let msg = request_builder::new_produce_request(topic_name, buffer.as_bytes());
        stdin_tx.send(msg).expect("Unable to send message");
    }
    //stdin_tx.send("0".as_bytes().to_vec()).expect("Unable to send message");
    // How do we gracefully close the connection??
    //Ok(())
}

fn stdout_processor(stdin_tx: UnboundedSender<Vec<u8>>, topic_name: &str, consumer_group: &str) -> io::Result<()> {
    // Can these requests be batched?
    let msg = request_builder::new_consume_message(topic_name, consumer_group);
    stdin_tx.send(msg).expect("Unable to send message");
    // This exits before it actaully recieves the messagee
    //thread::sleep(time::Duration::from_millis(5000));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    Builder::new().filter_level(LevelFilter::Info).init();    
    let matches = base_cli().get_matches();

    let stdin_tx: UnboundedSender<Vec<u8>>;
    let stdin_rx: UnboundedReceiver<Vec<u8>>;
    (stdin_tx , stdin_rx) = tokio::sync::mpsc::unbounded_channel();

    match matches.subcommand() {
        Some(("connect", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, stdin_rx).await;
                res.expect("Server crashed unexpectedly")
            });
            thinger(stdin_tx)
        },
        Some(("producer", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, stdin_rx).await;
                res.expect("Server crashed unexpectedly")
            });

            stdin_processor(stdin_tx, topic_name).expect("Unable to process messages");
            info!("Exiting...");
            return Ok(());
        },
        // Probably need to add another channel so we can block until we get a message back. This would simplify reading 
        // messages that are just randomly logged out. We need asyc behavior to run the server, however we need a synchronous model to be able
        // to reason about messages coming back and forth from the client. Serious refactoring needed.
        Some(("consumer", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");
            let consumer_group = sub_matches.get_one::<String>("CONSUMER_GROUP").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, stdin_rx).await;
                res.expect("Server crashed unexpectedly")
            });

            stdout_processor(stdin_tx, topic_name, consumer_group).expect("Unable to process consumer message");
            info!("Exiting...");
            return Ok(());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    } 

}
