mod key_value_store;
mod kvs_types;
mod packet;
mod socket;

use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use packet::*;
use tokio::net::*;
use futures::sink::SinkExt;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LinesCodec};

async fn handle_connection(mut sock: TcpStream) -> Result<(), PacketError> {
    let mut writer = Framed::new(sock, PacketCodec);
    let packet = Packet{
        packet_type: PacketType::TextPacket, content_length: 5usize, content: PacketBody::TextPacket("Hithe".to_string())
    };
    writer.send(packet).await?;
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

