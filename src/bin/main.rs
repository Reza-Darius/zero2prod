use tokio::net::TcpListener;
use zero2prod::{Database, load_config};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    let config = load_config()?;

    let db = Database::new(&config.db_url()).await?;

    zero2prod::run(listener, db).await?;
    Ok(())
}
