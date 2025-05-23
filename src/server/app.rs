use std::string::ToString;
use std::sync::Arc;
use futures::{StreamExt, TryFutureExt};
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use crate::common::{Document, KeyValueStore};
use crate::common::kvs_types::{DocType, KVSError};
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
        let fail_packet = Packet{
            packet_type: TextPacket,
            content: PacketBody::TextPacket(String::from("Request failed!")),
        };
        
        let ok_packet = Packet {
            packet_type: TextPacket,
            content: PacketBody::TextPacket(String::from("Ok")),
        };

        let pong = Packet{
            packet_type: TextPacket,
            content: PacketBody::TextPacket("Pong!".to_string()),
        };

        let mut socket = Socket::new(stream);
        let connect_info = Packet {
            packet_type: PacketType::TextPacket,
            content: PacketBody::TextPacket("Connected".to_string()),
        };
        socket.send(&connect_info).await?;
        
        loop {
            if let Some(packet) = socket.read_packet().await? {
                match packet.packet_type {
                    PacketType::GetRequest => {
                        let kvs = self.kvs.lock().await;
                        if let PacketBody::RequestBody { key, .. } = packet.content {
                            match kvs.get(&key) {
                                Err(e) => socket.send(&fail_packet).await?,
                                Ok(value) => {
                                    let as_string = value.data.to_string();
                                    let packet = Packet::new(
                                        TextPacket, 
                                        PacketBody::TextPacket(as_string),
                                    );
                                    socket.send(&packet).await?;
                                } 
                            } 
                        }
                    },
                    PacketType::SetRequest | PacketType::DelRequest => {
                        if let Err(e) = self.mutate_store(&packet).await {
                            socket.send(&fail_packet).await?;
                            println!("Failed to process a user's request!");
                        }
                        socket.send(&ok_packet).await?;
                    },
                    PacketType::PingRequest => {

                        socket.send(&pong).await?;
                    },
                    PacketType::TextPacket => {
                        let content = if let PacketBody::TextPacket(s) = packet.content {
                            s
                        } else {
                            "Invalid text packet".to_string()
                        };
                        println!("Client sent text packet w/ :\n {content}",);
                    },
                }
            }
        } 
    }
    
    async fn mutate_store(&self, packet: &Packet) -> Result<(), KVSError> {
        match &packet.content {
            PacketBody::RequestBody{key, new_value} => {
                // acquire the kvs mutex
                let kvs_ref = Arc::clone(&self.kvs);
                let mut kvs = kvs_ref.lock().await;
                match packet.packet_type {
                    PacketType::SetRequest => {
                        if let Some(value) = new_value {
                            kvs.put(key.to_string(), Document{
                                data: DocType::Raw(value.to_string()),
                            })?;
                        }
                    },
                    PacketType::DelRequest => {
                        kvs.del(key)?;
                    }
                    _ => {}
                }
                
                Ok(())
            },
            _ => { Ok(()) }
        }
    }
    
}