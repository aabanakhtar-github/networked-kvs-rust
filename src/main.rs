mod key_value_store;
mod kvs_types;

use std::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use key_value_store::*;
use kvs_types::*;
use tokio::net::*;

async fn handle_connection(mut sock: TcpStream) -> Result<(), Error> {
    let mut buf = vec![0u8; 1024];
    loop {
        let sz = sock
            .read(&mut buf)
            .await?;

        sock.write_all(&buf[0..sz]).await?;
        buf.fill(0);
    }
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

    Ok(())
}

