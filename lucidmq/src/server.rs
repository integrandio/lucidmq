//! A chat server that broadcasts a message to all connections.
//!
//! This is a simple line-based server which accepts WebSocket connections,
//! reads lines from those connections, and broadcasts the lines to all other
//! connected clients.
//!
//! You can test this out by running:
//!
//!     cargo run --bin server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --bin client ws://127.0.0.1:12345/
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex}
};

use futures::SinkExt;
use futures_channel::mpsc::{UnboundedSender};
use futures_util::{StreamExt};
use log::{error, info};

use tokio_tungstenite::{accept_async};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Result;

use crate::{Command, SenderType};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;


pub struct LucidServer {
    peer_map: PeerMap,
    address: String,
    sender: SenderType
}

impl LucidServer {
    pub fn new(sender: SenderType) -> LucidServer {
        let addr = "127.0.0.1:8080".to_string();
        LucidServer {
            peer_map: PeerMap::new(Mutex::new(HashMap::new())),
            address: addr,
            sender: sender
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<(), IoError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", self.address);
    
        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let cloned_server = Arc::clone(&self);
            tokio::spawn({
                async move{
                    cloned_server.accept_connection(stream, addr).await;
                }
            });
        }
        Ok(())
    }

    // async fn handle_connection(&self, raw_stream: TcpStream, addr: SocketAddr) {
    //     info!("Incoming TCP connection from: {}", addr);
    
    //     let ws_stream = tokio_tungstenite::accept_async(raw_stream)
    //         .await
    //         .expect("Error during the websocket handshake occurred");
    //     info!("WebSocket connection established: {}", addr);
    
    //     // Insert the write part of this peer to the peer map.
    //     let (tx, rx) = unbounded();
    //     self.peer_map.lock().unwrap().insert(addr, tx);
    
    //     let (outgoing, incoming) = ws_stream.split();
    
    //     let broadcast_incoming = incoming.try_for_each(|msg| {
    //         info!("Received a message from {}: {}", addr, msg.to_text().unwrap());
            
    //         let peers = self.peer_map.lock().unwrap();
    
    //         // We want to broadcast the message to everyone except ourselves.
    //         let broadcast_recipients =
    //             peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
    
    //         for recp in broadcast_recipients {
    //             recp.unbounded_send(msg.clone()).unwrap();
    //         }
    
    //         future::ok(())
    //     });
    
    //     let receive_from_others = rx.map(Ok).forward(outgoing);
    
    //     pin_mut!(broadcast_incoming, receive_from_others);
    //     future::select(broadcast_incoming, receive_from_others).await;
    
    //     info!("{} disconnected", &addr);
    //     self.peer_map.lock().unwrap().remove(&addr);
    // }


    async fn accept_connection(&self, stream: TcpStream, peer: SocketAddr) {
        if let Err(e) = self.handle_connection2(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            }
        }
    }
    
    async fn handle_connection2(&self, peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    
        info!("New WebSocket connection: {}", peer);
    
        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if msg.is_binary(){
                let command = parse_mesage(msg.to_text().unwrap());
                self.sender.send(command).await.unwrap();
                ws_stream.send(msg).await?;
            } else {
                print!("{:?}", msg)
            }
        }
    
        Ok(())
    }
}

pub fn parse_mesage(websocket_message: &str) -> Command {
    info!("{:?}", websocket_message);
    match websocket_message {
        "produce\n" => {
            Command::Produce{ key: "produce".to_string()}
        },
        "consume\n" => {
            Command::Consume{ key: "consume".to_string()}
        },
        "topic\n" => {
            Command::Topic{ key: "topic".to_string()}
        },
        _=> {
            info!("Cant parse thing....");
            Command::Invalid{ key: "invalid".to_string()}
        }
    }
    
}