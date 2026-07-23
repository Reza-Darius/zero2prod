use crate::{handler::*, server::AppState};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/health", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
