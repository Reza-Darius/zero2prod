use axum::{body::Body, response::Response};

pub async fn health_check() -> Response {
    Response::new(Body::from("im alive!"))
}
