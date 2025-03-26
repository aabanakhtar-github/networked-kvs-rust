use futures::SinkExt;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use crate::packet::{Packet, PacketCodec, PacketError};

#[derive(Debug, Error)]
enum NetworkError {
    #[error("Generic Network error: {0}")]
    GenericError(String),
    #[error("Packet Error")]
    PacketError(#[from] PacketError),
}

struct Socket {
    base: Framed<TcpStream, PacketCodec>,
}

impl Socket {
    pub fn new(stream: TcpStream) -> Socket {
       Socket{base: Framed::new(stream, PacketCodec)}
    }

    pub fn read_packet(&mut self) -> Result<Packet, PacketError> {
        todo!()
    }

    pub async fn send(&mut self, packet: Packet) -> Result<(), NetworkError> {
        self.base.send(packet).await?;
        Ok(())
    }
}