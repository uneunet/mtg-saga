use crate::middleware;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use usecase::UseCase;

pub fn routes(use_case: UseCase) -> Router {
    Router::new()
        .route("/", get(get_user_info))
        .layer(from_fn_with_state(use_case.clone(), middleware::auth))
        .with_state(use_case)
}

pub async fn get_user_info(
    State(use_case): State<UseCase>,
    Extension(user_id): Extension<String>,
) -> axum::response::Result<impl IntoResponse, StatusCode> {
    let user = use_case
        .user
        .find_by_id(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user.info))
}
