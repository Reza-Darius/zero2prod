use axum::{Form, extract::rejection::FormRejection, response::IntoResponse};
use reqwest::StatusCode;
use tracing::{error, info};

use crate::models::{MyError, User};

pub async fn health_check() -> impl IntoResponse {
    info!("new health check");

    "im alive!"
}

pub async fn subscribe(user: Result<User, MyError>) -> StatusCode {
    match user {
        Ok(user) => {
            info!(name = user.name, email = user.email, "new subscriber");
            StatusCode::OK
        }
        Err(_) => {
            StatusCode::BAD_REQUEST
        },
    }
}
