use std::io;
use tokio::net::TcpListener;

use tracing::info;

use crate::routes::routes;

pub async fn run(listener: TcpListener) -> io::Result<()> {
    info!("listening on {}", listener.local_addr()?);

    axum::serve::serve(listener, routes().into_make_service()).await
}
