use axum_server::Server;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

use tracing::info;

use crate::{database::Database, routes::routes};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppInner>,
}

struct AppInner {
    db: Database,
}

impl AppState {
    fn new(db: Database) -> Self {
        AppState {
            inner: Arc::new(AppInner { db }),
        }
    }
}

pub async fn run(listener: TcpListener, db: Database) -> io::Result<()> {
    let app = AppState::new(db);
    let routes = routes(app);

    info!("listening on {}", listener.local_addr()?);
    Server::<SocketAddr>::from_listener(listener)
        .serve(routes.into_make_service())
        .await
}
