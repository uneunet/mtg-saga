use crate::types::*;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{self, IntoResponse, Response},
};
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

pub async fn get_user_info(
    State(users): State<Collection<User>>,
    Extension(email): Extension<String>,
) -> axum::response::Result<response::Json<User>> {
    let user = users
        .find_one(doc! { "email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user))
}

pub async fn get_user_info_with_name(
    State(users): State<Collection<User>>,
    Path(name): Path<String>,
) -> axum::response::Result<response::Json<User>> {
    let email = users
        .find_one(doc! { "name": name })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap()
        .email;

    let user = users
        .find_one(doc! { "email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user))
}

pub async fn delete_user(
    State(users): State<Collection<User>>,
    Extension(email): Extension<String>,
) -> axum::response::Result<response::Response> {
    users
        .delete_one(doc! { "email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK.into_response())
}
