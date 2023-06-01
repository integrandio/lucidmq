mod tcp_client;
use std::io::Write;
use std::{thread, time};
use std::{net::SocketAddr};
use env_logger::Builder;
use log::{LevelFilter, info};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
mod request_builder;
pub mod lucid_schema_capnp;
mod cap_n_proto_helper;
mod cli_helper;
use std::io::{self, BufRead};

fn respond(line: &str) -> Result<Vec<u8>, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli_helper::interactive_cli()
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
            return Ok(request_builder::new_consume_message(topic_name, consumer_group, 1));
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
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "> ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

async fn interactive_handler(stdin_tx: UnboundedSender<Vec<u8>>, mut stdin_rx: UnboundedReceiver<String>) -> Result<(), String> {
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
        let response = stdin_rx.recv().await.expect("Unable to recieve message");
        write!(std::io::stdout(), "{}", response).map_err(|e| e.to_string())?;
        std::io::stdout().flush().map_err(|e| e.to_string())?;
    }
    info!("Exiting...");
    Ok(())
}

async fn stdin_processor(topic_name: &str, stdin_tx: UnboundedSender<Vec<u8>>, mut stdin_rx: UnboundedReceiver<String>) -> io::Result<()> {
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
        let response = stdin_rx.recv().await.expect("Unable to recieve message");
        write!(std::io::stdout(), "{}", response).expect("Unable to write message");
        std::io::stdout().flush().expect("Unable to flush message");
    }
    //stdin_tx.send("0".as_bytes().to_vec()).expect("Unable to send message");
    // How do we gracefully close the connection??
    //Ok(())
}

async fn stdout_processor(topic_name: &str, consumer_group: &str, stdin_tx: UnboundedSender<Vec<u8>>, mut stdin_rx: UnboundedReceiver<String>) -> io::Result<()> {
    loop {
        let msg = request_builder::new_consume_message(topic_name, consumer_group, 10000);
        stdin_tx.send(msg).expect("Unable to send message");
        let response = stdin_rx.recv().await.expect("Unable to recieve message");
        write!(std::io::stdout(), "{}", response).expect("Unable to write message");
        std::io::stdout().flush().expect("Unable to flush message");
    }
    //Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    Builder::new().filter_level(LevelFilter::Info).init();    
    let matches = cli_helper::base_cli().get_matches();

    let request_channel_sender: UnboundedSender<Vec<u8>>;
    let request_channel_reciever: UnboundedReceiver<Vec<u8>>;
    (request_channel_sender , request_channel_reciever) = tokio::sync::mpsc::unbounded_channel();
    let response_channel_sender: UnboundedSender<String>;
    let response_channel_reciever: UnboundedReceiver<String>;
    (response_channel_sender , response_channel_reciever) = tokio::sync::mpsc::unbounded_channel();

    match matches.subcommand() {
        Some(("connect", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, request_channel_reciever, response_channel_sender).await;
                res.expect("Server crashed unexpectedly")
            });
            interactive_handler(request_channel_sender, response_channel_reciever).await
        },
        Some(("producer", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, request_channel_reciever, response_channel_sender).await;
                res.expect("Server crashed unexpectedly")
            });

            stdin_processor(topic_name, request_channel_sender, response_channel_reciever).await.expect("Unable to process messages");
            info!("Exiting...");
            return Ok(());
        },
        Some(("consumer", sub_matches)) => {
            let address = sub_matches.get_one::<String>("ADDRESS").expect("required");
            let port = sub_matches.get_one::<String>("PORT").expect("required");
            let topic_name = sub_matches.get_one::<String>("TOPIC_NAME").expect("required");
            let consumer_group = sub_matches.get_one::<String>("CONSUMER_GROUP").expect("required");

            let connection_string: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
            info!("Connected to {}", connection_string.to_string());
            tokio::spawn(async move {
                let res = tcp_client::run_client(connection_string, request_channel_reciever, response_channel_sender).await;
                res.expect("Server crashed unexpectedly")
            });

            stdout_processor(topic_name, consumer_group, request_channel_sender, response_channel_reciever).await.expect("Unable to process consumer message");
            info!("Exiting...");
            return Ok(());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    } 

}
