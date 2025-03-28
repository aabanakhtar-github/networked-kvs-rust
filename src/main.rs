mod server;
mod client;
mod common;

use crate::common::socket::NetworkError;
use crate::server::app::Server;
use crate::client::app::Client; 

#[tokio::main]
async fn main() -> Result<(), NetworkError> {
    let usage = "Usage: kvs <server | client> <ip>";
    
    if std::env::args().count() == 0 {
        println!("{}", usage);
        return Ok(())
    }
    
    let mode = std::env::args()
        .nth(1)
        .unwrap_or("server".to_string());
    
    let ip = std::env::args()
        .nth(2)
        .unwrap_or("127.0.0.1:8080".to_string());
    
    match mode.as_str() {
       "server" => { 
           let mut server = Server::new(&ip);
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
