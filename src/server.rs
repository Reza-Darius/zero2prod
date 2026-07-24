use axum_server::Server;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

use tracing::info;

use crate::{database::Database, routes::routes};

#[derive(Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

struct AppInner {
    db: Database,
}

impl App {
    fn new(db: Database) -> Self {
        App {
            inner: Arc::new(AppInner { db }),
        }
    }

    pub fn db(&self) -> &Database {
        &self.inner.db
    }
}

pub async fn run(listener: TcpListener, db: Database) -> io::Result<()> {
    let app = App::new(db);
    let routes = routes(app);

    info!("listening on {}", listener.local_addr()?);
    Server::<SocketAddr>::from_listener(listener)
        .serve(routes.into_make_service())
        .await
}
