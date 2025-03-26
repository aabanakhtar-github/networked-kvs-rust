mod common;

use std::io::Error;
use tokio::net::TcpListener;
use tokio::net::*;
use common::{packet::*, socket::*, kvs_types::*, key_value_store::*};

async fn handle_connection(mut stream: TcpStream) -> Result<(), NetworkError> {
    let mut socket = Socket::new(stream);
    let packet = Packet{
        packet_type: PacketType::TextPacket, content_length: 5usize, content: PacketBody::TextPacket("Hi there!".to_string())
    };
    socket.send(packet).await?;
    return Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ip = std::env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8080".to_string());

    let connection_manager = TcpListener::bind(&ip).await?;

    loop {
        let (sock, _) = connection_manager.accept().await?;
        tokio::spawn(handle_connection(sock));
    }
}

