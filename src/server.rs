use axum_server::Server;
use std::{io, net::SocketAddr};
use tokio::net::TcpListener;

use tracing::info;

use crate::routes::routes;

pub async fn run(listener: TcpListener) -> io::Result<()> {
    info!("listening on {}", listener.local_addr()?);

    Server::<SocketAddr>::from_listener(listener)
        .serve(routes().into_make_service())
        .await
}
