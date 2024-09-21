use crate::types::*;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{self, IntoResponse, Response},
};
use mongodb::{bson::{oid::ObjectId, doc}, Collection};
use serde::{Deserialize, Serialize};

pub async fn get_user_info(
    State(states): State<DBStates>,
    Extension(user): Extension<ObjectId>,
) -> axum::response::Result<response::Json<UserInfo>> {
    let users = states.users;

    let user = users
        .find_one(doc! { "_id": user })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user.info))
}

pub async fn get_user_info_with_name(
    State(states): State<DBStates>,
    Path(email): Path<String>,
) -> axum::response::Result<response::Json<UserInfo>> {
    let users = states.users;

    let user = users
        .find_one(doc! { "info.email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    Ok(response::Json(user.info))
}

pub async fn delete_user(
    State(states): State<DBStates>,
    Extension(email): Extension<String>,
) -> axum::response::Result<response::Response> {
    let users = states.users;

    users
        .delete_one(doc! { "info.email": email })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK.into_response())
}
