use std::{collections::HashMap, io::Error as IoError, net::SocketAddr, str::FromStr, sync::Arc};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt,
};
use futures_util::StreamExt;
use log::{info, warn, error};

use tokio::sync::Mutex;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{accept_async, WebSocketStream};

use crate::{types::Command, RecieverType, SenderType};

type Tx = SplitSink<WebSocketStream<TcpStream>, Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// Struct that implements the websocket server
pub struct LucidServer {
    // A map containing all of the current connections to the server
    peer_map: PeerMap,
    // Address where the server is serving from
    address: String,
    // The channel to send updates to the broker
    sender: SenderType,
    // The channel to recieve updates from the broker
    reciever: RecieverType,
}

impl LucidServer {
    // Creates a new instance of a LucidServer
    pub fn new(sender: SenderType, reciever: RecieverType) -> LucidServer {
        let addr = "127.0.0.1:8080".to_string();
        LucidServer {
            peer_map: PeerMap::new(Mutex::new(HashMap::new())),
            address: addr,
            sender: sender,
            reciever: reciever,
        }
    }

    // Start the lucid server by listening for connections and responding to updates from the broker
    pub async fn start(self) -> Result<(), IoError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", self.address);

        let thing = Arc::new(self.peer_map.clone());

        tokio::spawn(async move { handle_responses(self.reciever, thing).await });

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let cloned_sender = self.sender.clone();
            let ws_stream = accept_async(stream).await.expect("Failed to accept");
            let (outgoing, incoming) = ws_stream.split();
            self.peer_map.lock().await.insert(addr, outgoing);

            tokio::spawn(async move {
                let res = handle_connection(incoming, cloned_sender, addr.to_string()).await;
                match res {
                    Err(e) => {
                        error!("{}", e)
                    }
                    Ok(_) => {},
                }
            });
        }
        Ok(())
    }

}

async fn handle_connection(
    mut incoming: SplitStream<WebSocketStream<TcpStream>>,
    sender: SenderType,
    addr: String
) -> Result<()> {
    while let Some(msg) = incoming.next().await {
        let msg = msg?;
        if msg.is_binary() {
            let command = parse_mesage(msg.to_text().unwrap(), addr.clone());
            sender.send(command).await.unwrap();
        } else {
            warn!("message is not binary: {:?}", msg)
        }
    }
    Ok(())
}

async fn handle_responses(mut reciever: RecieverType, peermap: Arc<PeerMap>) {
    while let Some(command) = reciever.recv().await {
        let id;
        let response_message: Message;
        match command {
            Command::Response { conn_id, key: thing , data: _} => {
                info!("{}", thing);
                response_message = Message::Text("hi".to_string());
                id = conn_id;
            }
            _ => {
                panic!("Command not good")
            }
        }
        let mut wing = peermap.lock().await;
        let outgoing = wing.get_mut(&id).expect("Key not found");
        let res = outgoing.send(response_message).await;
        match res {
            Err(e) => {
                error!("{}", e)
            }
            Ok(_) => {},
        }
    }
}

pub fn parse_mesage(websocket_message: &str, addr: String) -> Command {
    info!("{:?}", websocket_message);
    match websocket_message {
        "produce\n" => Command::Produce {
            conn_id: addr,
            key: "produce".to_string(),
            data: "data".as_bytes().to_vec()
        },
        "consume\n" => Command::Consume {
            conn_id: addr,
            key: "consume".to_string(),
            data: "data".as_bytes().to_vec()
        },
        "topic\n" => Command::Topic {
            conn_id: addr,
            key: "topic".to_string(),
            topic_name: "topic1".to_string()
        },
        _ => {
            info!("Cant parse message.... {}", websocket_message);
            Command::Invalid {
                key: "invalid".to_string(),
            }
        }
    }
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
