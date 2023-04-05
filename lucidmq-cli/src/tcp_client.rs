use log::{debug, error, info};
use tokio::sync::mpsc::UnboundedReceiver;
use std::{error::Error, net::SocketAddr};
use crate::cap_n_proto_helper;

use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub async fn run_client(server_addr: SocketAddr, stdin_rx: UnboundedReceiver<Vec<u8>>) -> Result<(), Box<dyn Error>> {

    let stream = TcpStream::connect(server_addr)?;
    debug!("connected: addr={}", stream.peer_addr().unwrap());
    
    // Reading from TCP stream is done in a thread
    tokio::spawn(read_from_stream(stream.try_clone().unwrap()));
    // Main thread handles writing to the QUIC stream
    write_to_stream(stream.try_clone().unwrap(), stdin_rx).await;
    info!("Cleaning up connection");
    stream.shutdown(Shutdown::Both).unwrap();
    drop(stream);
    info!("Closed");
    Ok(())
}

async fn write_to_stream(mut send: TcpStream, mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>) {
    loop {
        let message = rx.recv().await.expect("Unable to recieve message");
        if message.len() == 1 {
            break;
        }
        debug!("{:?}", message);
        send.write(&message).expect("Unable to send message");
    }
}

async fn read_from_stream(mut recv: TcpStream) {
    // This buffer is used to get the size of the actual message
    let mut buf = [0u8; 2];
    loop {
        let message_size_reader_result = recv.read(&mut buf);
        let _bytes_read = match message_size_reader_result {
            Ok(reader_bytes) => reader_bytes,
            Err(err) => {
                error!("{}", err);
                break;
            },
        };
        let message_size = u16::from_le_bytes(buf);

        let mut message_vec = vec![0u8; message_size.into()];
        let message_buff = &mut message_vec;

        let message_reader_result = recv.read(message_buff);
        let message_bytes_read = match message_reader_result {
            Ok(reader_bytes) => reader_bytes,
            Err(err) => {
                error!("{}", err);
                break;
            },
        };
        debug!("Second Bytes recieved {:?} size {}", message_buff, message_bytes_read);
        cap_n_proto_helper::parse_response(message_buff.to_vec());
    }
}
