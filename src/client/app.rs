use crate::common::socket::{NetworkError, Socket};
use tokio::net::TcpStream;
use std::io::{stdin, stdout, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::common::packet::PacketBody::RequestBody;
use crate::common::packet::{Packet, PacketBody, PacketType};
use crate::common::socket::NetworkError::GenericError;

pub struct Client {
    connection: Socket
}

impl Client {
    pub async fn new(ip: &str) -> Result<Client, NetworkError> {
        let connection = TcpStream::connect(ip).await.map_err(|_| NetworkError::ConnectionError)?;
        Ok(Client {
            connection: Socket::new(connection)
        })
    }

    pub async fn main(&mut self) -> Result<(), NetworkError> {
        let stdin = tokio::io::stdin(); 
        let mut reader = BufReader::new(stdin);
        
        loop {
            print!("KVS > ");
            stdout().flush()?;
            let mut input = String::new();
            
            if let Err(e) = reader.read_line(&mut input).await {
                break Err(NetworkError::StdError(e));
            } 

            self.handle_prompt(&input).await?;
        }
    }
    
    async fn handle_prompt(&mut self, prompt: &str) -> Result<(), NetworkError> {
        let args = prompt.trim().split(' ').collect::<Vec<&str>>();
        let method: String;
        let key: String;
        let mut value: Option<String> = None;
        println!("{}", args.as_slice().join(" ")); 
        match args.len() {
            2 | 3 => {
                method = args[0].to_string();
                key = args[1].to_string();
                if args.len() == 3 {
                    value = Some(args[2].to_string());
                }
            }
            _ => {
                println!("Input Error: <METHOD> <KEY> <Optional: Value>");
                return Ok(());
            }
        }

        let result = self.send_request(&method, &key, &value).await;
        match result {
            Err(_) => println!("Error sending request"),
            _ => {}
        };
        
        self.handle_response().await
    }
    
    async fn handle_response(&mut self) -> Result<(), NetworkError> {
        println!("Handling reponse!");
        if let Some(packet) = self.connection.read_packet().await? {
            match packet.content {
                PacketBody::TextPacket(data) => {
                    println!("Received data from server: {}", &data);
                }, 
                _ => {}
            } 
        }
        println!("Dopne hadling!"); 
        Ok(()) 
    } 
    
    async fn send_request(&mut self, method: &str, key_value: &str, doc: &Option<String>) -> Result<(), NetworkError> {
        let p_type = match method {
            "GET" => PacketType::GetRequest, 
            "SET" => PacketType::SetRequest,
            "PING" => PacketType::PingRequest,
            "DEL" => PacketType::DelRequest,
            _ => return Err(GenericError("Invalid method!".to_string()))
        };
        
        let p_body = PacketBody::RequestBody {
            key: key_value.to_string(), 
            new_value: doc.clone() 
        };

        self.connection.send(Packet::new(
            p_type,
            p_body,
        )).await 
    }
}