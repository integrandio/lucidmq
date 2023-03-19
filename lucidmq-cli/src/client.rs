use quinn::{ClientConfig, Endpoint, SendStream};
use tokio::{io::AsyncReadExt};
use std::{error::Error, net::SocketAddr, sync::Arc};

use crate::cap_n_proto_helper;
use crate::request_builder;


pub async fn run_client(server_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let client_cfg = configure_client();
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse().unwrap())?;
    endpoint.set_default_client_config(client_cfg);

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());

    let (stdin_tx, stdin_rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(read_stdin(stdin_tx));
    
    let (send, mut recv) = connection.open_bi().await.expect("Unable to open bidirection stream");

    tokio::spawn(write_to_stream(send, stdin_rx));
    
    // This buffer is used to get the size of the actual message
    let mut buf = [0u8; 2];
    loop {
        let bytes_read = recv.read(&mut buf).await.expect("unable to read message");
        let message_size: u16 = match bytes_read {
            Some(_total) => {
                let message_size = u16::from_le_bytes(buf);
                message_size
            },
            None => {
                continue;
            }
        };
        let mut message_vec = vec![0u8; message_size.into()];
        let message_buff = &mut message_vec;

        let message_bytes_read = recv.read(message_buff).await.expect("unable to read message");
        match message_bytes_read {
            Some(total) => {
                println!("Second Bytes recieved {:?} size {}", message_buff, total);
            },
            None => {
                continue;
            }
        };
        cap_n_proto_helper::parse_response(message_buff.to_vec());
    }

    connection.close(0u32.into(), b"done");

    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);
    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];

        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);

        let s = String::from_utf8_lossy(&buf);
        let request= parse_mesage(&s.to_string());
        tx.send(request).expect("Unable to send message");
    }
}

async fn write_to_stream(mut send: SendStream, mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>) {
    loop {
        let message = rx.recv().await.expect("Unable to recieve message");
        println!("{:?}", message);
        send.write(&message).await.unwrap();//.expect("unable to write request bytes");
        //send.finish().await.expect("unable to finish connection");
    }
}


pub fn parse_mesage(console_msg: &str) -> Vec<u8> {
    match console_msg {
        "produce\n" => {
            request_builder::new_produce_request()
        },
        "consume\n" => {
            request_builder::new_consume_message()
        }
        "topic_create\n" =>{
            request_builder::new_topic_request_create()
        },
        "topic_describe\n" =>{
            request_builder::new_topic_request_describe()
        },
        "topic_delete\n" =>{
            request_builder::new_topic_request_delete()
        },
        _ => {
            panic!("response not recognized")
        }
    }
}


/// Dummy certificate verifier that treats any certificate as valid.
/// NOTE, such verification is vulnerable to MITM attacks, but convenient for testing.
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}