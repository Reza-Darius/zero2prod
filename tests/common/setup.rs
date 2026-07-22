use std::net::SocketAddr;
use tokio::net::TcpListener;

use reqwest::{Client, ClientBuilder};

/// returns the address of the server on a randomly given port, use that to build a response
pub async fn start_server() -> Result<SocketAddr, anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(zero2prod::run(listener));

    Ok(addr)
}

pub fn get_client() -> Client {
    ClientBuilder::new().build().unwrap()
}
