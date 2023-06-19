/*
This is a quic implementation of the lucidmq server.
For now it will not be supported until tcp has been implemented. May be interesting to come and integrate.
 */
use log::{info, error, warn};
use quinn::{Endpoint, ServerConfig, SendStream};
use tokio::sync::Mutex;
use std::process::exit;
use std::time::Duration;
use std::{error::Error, net::SocketAddr, sync::Arc, collections::HashMap};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::cap_n_proto_helper::parse_request;

use crate::types::Command;
use crate::{types::{SenderType, RecieverType }};

type PeerMap = Arc<Mutex<HashMap<String, SendStream>>>;

pub struct LucidQuicServer {
    peer_map: PeerMap,
    address: SocketAddr,
    sender: SenderType,
    reciever: RecieverType
}

impl LucidQuicServer {
    pub fn new(sender: SenderType, reciever: RecieverType) -> LucidQuicServer {
        let addr = "127.0.0.1:5000".parse().unwrap();
        LucidQuicServer { 
            peer_map: PeerMap::new(Mutex::new(HashMap::new())),
            address: addr,
            sender: sender,
            reciever: reciever
        }
    }

    /// Runs a QUIC server bound to given address.
    pub async fn run_server(self) {
        info!("Server Listening on {}", self.address.to_string());
        let (endpoint, _server_cert) = make_server_endpoint(self.address).unwrap();
        
        let arc_peer_map = Arc::new(self.peer_map.clone());
        tokio::spawn(async move {
            handle_responses(self.reciever, arc_peer_map).await;
        });
        
        // accept a single connection
        while let Some(conn) = endpoint.accept().await {
            info!(
                "connection accepted: addr={}",
                conn.remote_address()
            );
            let cloned_sender = self.sender.clone();
            let arc_peer_map = Arc::new(self.peer_map.clone());
            tokio::spawn(async move {
                handle_connection(conn, arc_peer_map, cloned_sender).await;
            });
        }
    }
}


async fn handle_connection(conn: quinn::Connecting, peermap: Arc<PeerMap>, sender: SenderType) {
    let connection = conn.await.expect("Unble to get connection");

    while let Ok((send_stream, recv_stream)) = connection.accept_bi().await {
        let id = generate_connection_string();
        peermap.lock().await.insert(id.clone(), send_stream);
        handle_request(id.clone(), recv_stream, &sender).await;
        //If handle request is exited, we can remove the connection from our map
        peermap.lock().await.remove(&id);
    }
    info!("Closing connection");
    connection.close(0u32.into(), b"done");

    // async {
    //     info!("running");
    //     loop {
    //         info!("running0");
    //         let stream = connection.accept_bi().await;
    //         let stream = match stream {
    //             Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
    //                 info!("connection closed");
    //                 return Ok(());
    //             }
    //             Err(e) => {
    //                 return Err(e);
    //             }
    //             Ok(s) => s,
    //         };

    //         let id = generate_connection_string();
    //         peermap.lock().await.insert(id.clone(), stream.0);
    //         handle_request(id, stream.1).await;
    //         //sender.send(command).await.expect("Unable to send message");
    //     }
    // }.await;
}

async fn handle_request(conn_id: String, mut recv: quinn::RecvStream, sender: &SenderType) {
    let mut buf = [0u8; 2];
    loop {
        let bytes_read = recv.read(&mut buf).await.expect("unable to read message");
        let message_size: u16 = match bytes_read {
            Some(total) => {
                info!("First Bytes recieved {:?} size {}", buf, total);
                let message_size = u16::from_le_bytes(buf);
                message_size
            },
            None => {
                warn!("No new bytes in stream, exiting request stream");
                recv.stop(0u32.into()).unwrap_or_else(|err| {
                    warn!("{}", err)
                });
                break;
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
                warn!("No new bytes in stream, exiting request stream");
                recv.stop(0u32.into()).unwrap_or_else(|err| {
                    warn!("{}", err)
                });
                break;
            }
        };
        let msg = parse_request(conn_id.clone(), message_buff.clone());
        sender.send(msg).await.expect("Unble to send message");
    }
}

fn generate_connection_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    return rand_string;
}

async fn handle_responses(mut reciever: RecieverType, peermap: Arc<PeerMap>) {
    while let Some(command) = reciever.recv().await {
        info!("Recieved message");
        let id;
        let response_message: Vec<u8>;
        match command {
            Command::Response { conn_id, capmessagedata } => {
                response_message = capmessagedata;
                id = conn_id;

            }
            _ => {
                error!("Command not good");
                exit(0)
            }
        }
        let mut wing = peermap.lock().await;
        let outgoing = wing.get_mut(&id).expect("Key not found");
        outgoing.write(&response_message).await.expect("Unable to write response to buffer");
        //outgoing.finish().await.expect("unable to finish stream");
    }
}


// Helper Server endpoints
fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Endpoint, Vec<u8>), Box<dyn Error>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_idle_timeout(Some(quinn::VarInt::from_u32(300_000).into()))
        .keep_alive_interval(Some(Duration::from_secs(60)))
        .max_concurrent_uni_streams(255_u8.into());

    Ok((server_config, cert_der))
}