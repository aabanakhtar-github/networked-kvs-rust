use crate::common::socket::{NetworkError, Socket};
use tokio::net::TcpStream;
use std::io::{stdin, stdout, Write};
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

    pub async fn main(&self) -> Result<(), NetworkError> {
        loop {
            println!("KVS> ");
            stdout().flush()?;
            let mut input = String::new(); 
            stdin().read_line(&mut input).map_err(|_| GenericError("Invalid input!".to_string()))?; 
            
            let args = input.trim().split(' ').collect::<Vec<&str>>();
            let method: String;
            let key: String; 
            let mut value: Option<String> = None;
            
            match args.len() {
                2 | 3 => {
                    method = args[0].to_string();
                    key = args[1].to_string();
                    if args.len() > 2 {
                        value = Some(args[2].to_string());
                    } 
                } 
                _ => {
                    println!("Input Error: <METHOD> <KEY> <Optional: Value>");
                    continue; 
                } 
            }
            
            self.send_request(&key, &method, &value).await?;
        }
    }
    
    async fn send_request(&self, key: &str, value: &str, doc: &Option<String>) -> Result<(), NetworkError> {
        Ok(())
    }
}