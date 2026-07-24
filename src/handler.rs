use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use tracing::{error, info};

use crate::{
    models::{User, UserFormError},
    server::App,
};

pub async fn health_check() -> impl IntoResponse {
    info!("new health check");

    "im alive!"
}

pub async fn new_sub(
    app: State<App>,
    user: Result<User, UserFormError>,
) -> Result<StatusCode, StatusCode> {
    match user {
        Ok(user) => {
            info!(name = user.name, email = user.email, "new subscriber");
            app.db()
                .new_sub(&user.name, &user.email)
                .await
                .inspect_err(|e| error!(%e))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(StatusCode::OK)
        }
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[axum::debug_handler]
pub async fn get_subs(app: State<App>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = app
        .db()
        .get_subs()
        .await
        .inspect_err(|e| error!(%e))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
