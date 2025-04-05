use std::sync::Arc;
use futures::{StreamExt, TryFutureExt};
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use crate::common::KeyValueStore;
use crate::common::packet::*;
use crate::common::packet::PacketType::TextPacket;
use crate::common::socket::{NetworkError, Socket};

pub struct Server {
    kvs: Arc<Mutex<KeyValueStore>>,
    ip: String,
}

impl Server {
    pub fn new(ip: &str) -> Self {
        Server {
            ip: ip.to_string(),
            kvs: Arc::new(Mutex::new(KeyValueStore::new())),
        }
    }

    pub async fn main(self: &Arc<Self>) -> Result<(), NetworkError>  {
        let listener = TcpListener::bind(&self.ip)
            .map_err(|_| NetworkError::ConnectionError).await?;
        
        
        loop {
            let (stream, _) = listener.accept().
                map_err(|_| NetworkError::ConnectionError).await?;
            
            let server_ref = Arc::clone(&self);
            
            tokio::spawn(async move { 
                server_ref.handle_connection(stream).
                    map_err(|_| NetworkError::GenericError("Failed to spawn task for client!".to_string())).await; 
            });
        }
    }

    async fn handle_connection(&self, stream: TcpStream) -> Result<(), NetworkError> {
        let mut socket = Socket::new(stream);
        let packet = Packet {
            packet_type: PacketType::TextPacket,
            content: PacketBody::TextPacket("Server received connection!".to_string()),
        };
        socket.send(packet).await?;
        
        loop {
            if let Some(packet) = socket.read_packet().await? {
                match packet.packet_type {
                    PacketType::GetRequest => {
                        socket.send(packet).await?;
                    },
                    PacketType::SetRequest => {
                        
                    },
                    PacketType::PingRequest => {
                        let pong = Packet{
                            packet_type: TextPacket,
                            content: PacketBody::TextPacket("Pong!".to_string()),
                        };
                        socket.send(pong).await?;
                    },
                    PacketType::TextPacket => {
                        let content = if let PacketBody::TextPacket(s) = packet.content {
                            s
                        } else {
                            "Invalid text packet".to_string()
                        };
                        println!("Client sent text packet w/ :\n {content}",);
                    },
                    PacketType::DelRequest => todo!()
                }
            }
        } 
    }
    
    async fn mutate_store(packet: &Packet) -> Result<(), NetworkError> {
        match &packet.content {
            PacketBody::TextPacket(s) => { Ok(( ))}
            _ => { Ok (( ))}
        }
    }
    
}