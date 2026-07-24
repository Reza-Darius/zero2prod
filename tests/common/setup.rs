use std::net::SocketAddr;
use test_containers_util::sqlx_pg::PostgresTestDb;
use tokio::net::TcpListener;

use reqwest::{Client, ClientBuilder};
use zero2prod::Database;

/// returns the address of the server on a randomly given port, use that to build a response
/// the test db gets deleted when the guard is dropped
pub async fn start_server() -> Result<(SocketAddr, TestDbGuard), anyhow::Error> {
    let db = new_test_db().await;

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    tokio::spawn(zero2prod::run(listener, db.get_db()));

    Ok((addr, db))
}

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub struct TestDbGuard {
    guard: PostgresTestDb,
}

impl TestDbGuard {
    pub fn get_db(&self) -> Database {
        let pool = self.guard.pool();
        Database::from(pool)
    }
}

pub async fn new_test_db() -> TestDbGuard {
    TestDbGuard {
        guard: PostgresTestDb::create("zero2prod-test", &MIGRATOR, None, None).await,
    }
}

pub fn get_client() -> Client {
    ClientBuilder::new().build().unwrap()
}
