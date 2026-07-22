use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::info;

use crate::models::{UserFormError, User};

pub async fn health_check() -> impl IntoResponse {
    info!("new health check");

    "im alive!"
}

pub async fn subscribe(user: Result<User, UserFormError>) -> axum::response::Result<StatusCode> {
    match user {
        Ok(user) => {
            info!(name = user.name, email = user.email, "new subscriber");
            Ok(StatusCode::OK)
        }
        Err(_) => Err(StatusCode::BAD_REQUEST.into()),
    }
}
