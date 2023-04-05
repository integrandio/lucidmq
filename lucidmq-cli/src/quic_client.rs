use log::{debug, error, info};
use quinn::{ClientConfig, Endpoint, SendStream, RecvStream};
use tokio::sync::mpsc::UnboundedReceiver;
use std::{error::Error, net::SocketAddr, sync::Arc};
use crate::cap_n_proto_helper;

pub async fn run_client(server_addr: SocketAddr, stdin_rx: UnboundedReceiver<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let client_cfg = configure_client();
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse().unwrap())?;
    endpoint.set_default_client_config(client_cfg);

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    debug!("connected: addr={}", connection.remote_address());
    
    let (send, recv) = connection.open_bi().await.expect("Unable to open bidirection stream");

    // Reading from QUIC stream is done in a thread
    tokio::spawn(read_from_stream(recv));
    // Main thread handles writing to the QUIC stream
    write_to_stream(send, stdin_rx).await;
    info!("Cleaning up connection");
    connection.close(0u32.into(), b"done");
    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);
    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;
    info!("Closed");
    Ok(())
}

async fn write_to_stream(mut send: SendStream, mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>) {
    loop {
        let message = rx.recv().await.expect("Unable to recieve message");
        if message.len() == 1 {
            break;
        }
        debug!("{:?}", message);
        send.write(&message).await.unwrap();//.expect("unable to write request bytes");
    }
    send.finish().await.expect("unable to finish connection");
}

async fn read_from_stream(mut recv: RecvStream) {
    // This buffer is used to get the size of the actual message
    let mut buf = [0u8; 2];
    loop {
        let message_size_reader_result = recv.read(&mut buf).await;
        let bytes_read = match message_size_reader_result {
            Ok(reader_bytes) => reader_bytes,
            Err(err) => {
                error!("{}", err);
                recv.stop(0u32.into()).unwrap();
                break;
            },
        };
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

        let message_reader_result = recv.read(message_buff).await;
        let message_bytes_read = match message_reader_result {
            Ok(reader_bytes) => reader_bytes,
            Err(err) => {
                error!("{}", err);
                recv.stop(0u32.into()).unwrap();
                break;
            },
        };
        match message_bytes_read {
            Some(total) => {
                debug!("Second Bytes recieved {:?} size {}", message_buff, total);
            },
            None => {
                continue;
            }
        };
        cap_n_proto_helper::parse_response(message_buff.to_vec());
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