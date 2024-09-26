use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use usecase::UseCase;

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct SignUpForm {
    pub name: String,
    pub user_id: String,
    pub email: String,
    pub password: String,
}

pub fn routes(use_case: UseCase) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(use_case)
}

pub async fn signup(
    State(use_case): State<UseCase>,
    Json(user): Json<SignUpForm>,
) -> axum::response::Result<(), StatusCode> {
    use_case
        .user
        .create(user.name, user.email, user.user_id, user.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn login(
    State(use_case): State<UseCase>,
    Json(credentials): Json<Credentials>,
) -> axum::response::Result<Response, StatusCode> {
    if let Some(user) = use_case
        .user
        .find_by_email(credentials.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        if user.verify_password(credentials.password) {
            let session = use_case
                .session
                .create(user.info.user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(session.token.into_response())
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
