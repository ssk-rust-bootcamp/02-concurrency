use anyhow::Result;
use std::net::SocketAddr;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, addr).await {
                warn!("Error processing connection: {}", e);
            }
        });
    }
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                info!("connection closed: {}", raddr);
                break;
            }
            Ok(n) => {
                info!("received {} bytes from {}", n, raddr);
                let line = String::from_utf8_lossy(&buf[..n]);
                info!("received line: {}", line);
                stream.write_all(b"OK").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                info!("read error: {}", e);
                return Err(e.into());
            }
        }
    }
    warn!("connection closed: {}", raddr);
    Ok(())
}
