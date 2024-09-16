use crate::types::*;
use axum::{
    extract::{Extension, State, Path},
    http::StatusCode,
    response::{self, IntoResponse, Response},
};
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

pub async fn get_user_info(
    State(collection): State<Collection<User>>,
    Extension(email): Extension<String>,
) -> axum::response::Result<response::Json<User>> {
    let user = collection
        .find_one(doc! { "email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user))
}

pub async fn get_user_info_with_name(
    State(collection): State<Collection<User>>,
    Path(name): Path<String>
) -> axum::response::Result<response::Json<User>> {
    let email = collection
        .find_one(doc! { "name": name })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap()
        .email;

    let user = collection
        .find_one(doc! { "email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user))
}

pub async fn delete_user(
    State(collection): State<Collection<User>>,
    Extension(email): Extension<String>,
) -> axum::response::Result<response::Response> {
    collection.delete_one(doc! { "email": email })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK.into_response())
}
