mod key_value_store;
mod kvs_types;
mod packet;

use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use packet::*;
use tokio::net::*;
use crate::packet::PacketType::Hello;

async fn handle_connection(mut sock: TcpStream) -> Result<(), PacketError> {
    loop {
        let mut buf = vec![0; 1024];
        let data = sock.read(&mut buf).await?;
        let packet = Packet{
            packet_type: Hello,
            content_length: buf.len(),
            content: PacketBody::TextPacket(String::from_utf8(buf)?)
        };
        let buf: Vec<u8> = Packet::try_into(packet)?;
        sock.write(buf.as_slice()).await?;
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
}

