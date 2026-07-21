use std::net::SocketAddr;
use tokio::net::TcpListener;

use reqwest::{Client, ClientBuilder};

pub async fn start_server() -> Result<SocketAddr, anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(zero2prod::run(listener));

    Ok(addr)
}

pub fn get_client() -> Client {
    ClientBuilder::new().build().unwrap()
}
