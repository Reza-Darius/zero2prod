use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:8000").await?;

    zero2prod::run(listener).await.map_err(Into::into)
}
