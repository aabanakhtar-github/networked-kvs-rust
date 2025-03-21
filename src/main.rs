mod key_value_store;
mod kvs_types;
mod packet;

use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use key_value_store::*;
use kvs_types::*;
use packet::*;
use tokio::net::*;
use crate::packet::PacketType::Hello;

async fn handle_connection(mut sock: TcpStream) -> Result<(), PacketError> {
    loop {
        let (mut reader, mut writer) = sock.split();
        let mut buf_reader = BufReader::new(&mut reader);
        let mut buf_writer = BufWriter::new(&mut writer);
        let packet = Packet{packet_type: Hello, content_length: 5, content: PacketBody::TextPacket("Hello".to_string())};
        packet.prep_and_send(&mut buf_writer).await?;
    }
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

    Ok(())
}

