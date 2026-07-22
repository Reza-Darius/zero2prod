use axum::{
    extract::{FromRequest, Request},
    http::Response,
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub struct UserFormError;

impl IntoResponse for UserFormError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(axum::body::Body::empty())
            .unwrap()
    }
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = UserFormError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Form::<User>::from_request(req, state).await {
            Ok(user) => Ok(user.0),
            Err(_) => Err(UserFormError),
        }
    }
}
