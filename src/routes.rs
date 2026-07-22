use crate::handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
}
