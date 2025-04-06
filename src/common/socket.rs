use futures::SinkExt;
use futures::StreamExt;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use crate::common::packet::{Packet, PacketError, PacketCodec, };

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("standard error")]
    StdError(#[from] std::io::Error),
    #[error("Generic Network error: {0}")]
    GenericError(String),
    #[error("Packet Error")]
    PacketError(#[from] PacketError),
    #[error("Connection Error")]
    ConnectionError,
}

pub struct Socket {
    base: Framed<TcpStream, PacketCodec>,
}

impl Socket {
    pub fn new(stream: TcpStream) -> Socket {
       Socket{base: Framed::new(stream, PacketCodec)}
    }

    pub async fn read_packet(&mut self) -> Result<Option<Packet>, NetworkError> {
        match self.base.next().await {
            Some(packet_result) => {
                match packet_result {
                    Ok(p) => Ok(Some(p)),
                    Err(e) => Err(NetworkError::from(e))
                }
            },
            None => Ok(None)
        }
    }

    pub async fn send(&mut self, packet: &Packet) -> Result<(), NetworkError> {
        self.base.send(packet.clone()).await?;
        Ok(())
    }
}