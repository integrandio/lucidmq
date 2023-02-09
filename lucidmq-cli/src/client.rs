use log::info;
use quinn::{ClientConfig, Endpoint, SendStream};
use tokio::{io::AsyncReadExt, sync::mpsc::UnboundedSender};
use std::{error::Error, net::SocketAddr, sync::Arc};


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
    
    let (mut send, mut recv) = connection.open_bi().await.expect("Unable to open bidirection stream");

    tokio::spawn(write_to_stream(send, stdin_rx));
    
    let mut buf = vec![0u8; 5];
    let response_buffer = &mut buf;
    loop {
        recv.read(response_buffer).await.expect("unable to read message");

        println!("{}", std::str::from_utf8(response_buffer).expect("unable to convert"));
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
async fn read_stdin(tx: tokio::sync::mpsc::UnboundedSender<String>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];

        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);

        let s = String::from_utf8_lossy(&buf);
        tx.send(s.to_string()).expect("Unable to send message");
    }
}

async fn write_to_stream(mut send: SendStream, mut rx: tokio::sync::mpsc::UnboundedReceiver<String>) {
    loop {
        let message = rx.recv().await.expect("Unable to recieve message");
        let request_bytes = &message.as_bytes();
        send.write_all(request_bytes).await.expect("unable to write request bytes");
        send.finish().await.expect("unable to finish connection");
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