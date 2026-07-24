use crate::{handler::*, server::App};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn routes<S>(state: App) -> Router<S> {
    Router::new()
        .route("/health", get(health_check))
        .route("/subscriptions", post(new_sub))
        .route("/subscriptions", get(get_subs))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
