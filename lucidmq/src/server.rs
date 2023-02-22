use log::{info, error};
use quinn::{Endpoint, ServerConfig, SendStream};
use tokio::sync::Mutex;
use std::process::exit;
use std::{error::Error, net::SocketAddr, sync::Arc, collections::HashMap};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::{types::{SenderType, RecieverType, Command, Payload}};

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
                "[server] connection accepted: addr={}",
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
    loop {
        info!("Creating bidirectional stream");
        let (send_stream, recv_stream): (quinn::SendStream, quinn::RecvStream) = connection.accept_bi().await.expect("Unable to create a bidirection connection");
        let id = generate_connection_string();
        peermap.lock().await.insert(id.clone(), send_stream);
        let thing = handle_request(id, recv_stream).await;
        sender.send(thing).await.expect("Unable to send message");
    }
}

async fn handle_responses(mut reciever: RecieverType, peermap: Arc<PeerMap>) {
    while let Some(command) = reciever.recv().await {
        info!("Recieved message");
        let id;
        let response_message: Vec<u8>;
        match command {
            Command::Response (payload) => {
                info!("{}", payload.message);
                response_message = payload.data;
                id = payload.conn_id;
            }
            _ => {
                error!("Command not good");
                exit(0)
            }
        }
        let mut wing = peermap.lock().await;
        let outgoing = wing.get_mut(&id).expect("Key not found");
        outgoing.write_all(&response_message).await.expect("Unable to write response to buffer");
        outgoing.finish().await.expect("unable to finish stream");
    }
}

async fn handle_request(conn_id: String, recv_stream: quinn::RecvStream) -> Command {
    let req = recv_stream
        .read_to_end(64 * 1024)
        .await.expect("Failed reading request {}");
    let s = String::from_utf8_lossy(&req);
    let msg = parse_mesage(&s, conn_id);
    info!("{:?}", req);
    return msg;
}

fn generate_connection_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    return rand_string;
}

pub fn parse_mesage(websocket_message: &str, conn_id: String) -> Command {
    info!("{:?}", websocket_message);
    match websocket_message {
        "produce\n" => {
            let payload = Payload {
                conn_id: conn_id,
                message: "produce".to_string(),
                data: "data".as_bytes().to_vec()
            };
            Command::Produce(payload)
        },
        "consume\n" => {
            let payload = Payload {
                conn_id: conn_id,
                message: "consume".to_string(),
                data: "data".as_bytes().to_vec()
            };
            Command::Consume(payload)
        }
        "topic\n" =>{
            let payload = Payload {
                conn_id: conn_id,
                message: "consume".to_string(),
                data: "data".as_bytes().to_vec()
            };
            Command::Topic(payload)
        },
        _ => {
            info!("Cant parse message.... {}", websocket_message);
            Command::Invalid {
                message: "invalid".to_string(),
            }
        }
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
        .max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}