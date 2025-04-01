use std::sync::Arc;
use futures::{StreamExt, TryFutureExt};
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use crate::common::KeyValueStore;
use crate::common::packet::*;
use crate::common::socket::{NetworkError, Socket};

pub struct Server {
    kvs: Arc<Mutex<KeyValueStore>>,
    ip: String,
}

impl Server {
    pub fn new(ip: &str) -> Self {
        Server {
            ip: ip.to_string(),
            kvs: std::sync::Arc::new(tokio::sync::Mutex::new(KeyValueStore::new())),
        }
    }

    pub async fn main(&mut self) -> Result<(), NetworkError>  {
        let listener = TcpListener::bind(&self.ip).map_err(|_| NetworkError::ConnectionError).await?;

        loop {
            let (stream, _) = listener.accept().map_err(|_| NetworkError::ConnectionError).await?;
            tokio::spawn(Self::handle_connection(stream));
        }
    }

    async fn handle_connection(stream: TcpStream) -> Result<(), NetworkError> {
        let mut socket = Socket::new(stream);
        let packet = Packet {
            packet_type: PacketType::TextPacket,
            content: PacketBody::TextPacket("Server recieved connection!".to_string()),
        };
        socket.send(packet).await?;
        
        loop {
            if let Some(packet) = socket.read_packet().await? {
                match packet.packet_type {
                    PacketType::GetRequest => {
                        println!("Recieved get request!");
                        socket.send(packet).await?;
                    }
                    _ =>  println!("Received a stray packet.") 
                } 
            }
        } 
    }
}