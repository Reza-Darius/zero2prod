use crate::handler::*;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}
