mod server;
mod client;
mod common;

use std::sync::Arc;
use crate::common::socket::NetworkError;
use crate::server::app::Server;
use crate::client::app::Client; 

#[tokio::main]
async fn main() -> Result<(), NetworkError> {
    let usage = "Usage: kvs <server | client> <ip>";
    let args: Vec<String> = std::env::args().collect();
    
    let mode = &args[1];
    let ip = &args[2]; 
    println!("{}", args[1]); 
    match mode.as_str() {
       "server" => { 
           let server = Arc::new(Server::new(&ip));
           server.main().await
       }
       "client" => {
            let mut client = Client::new(&ip).await?;
            client.main().await
        }
       _ => {
           println!("{}", usage);
           Ok(())
       } 
    }
    
}
