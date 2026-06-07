use std::{net::SocketAddr, str::FromStr};

use axum::{Router, body::Body, response::Response, routing::get};
use tracing::error;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/health_check", get(health_check));
    let addr = "0.0.0.0:8000";

    let _ = axum_server::bind(SocketAddr::from_str(addr).unwrap())
        .serve(app.into_make_service())
        .await
        .inspect_err(|e| error!(%e));
}

async fn health_check() -> Response {
    Response::new(Body::from("im alive!"))
}
