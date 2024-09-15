use crate::types::*;
use axum::{
    extract::{Extension, State},
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
