use crate::types::*;
use axum::{
    extract::Request,
    extract::{FromRequestParts, Json, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    RequestExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use jwt_simple::prelude::*;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::env;

pub async fn sign_up(
    State(collection): State<Collection<User>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    if user.name.is_empty() || user.email.is_empty() || user.password.is_empty() {
        return StatusCode::PRECONDITION_FAILED.into_response();
    }
    println!("{:?}", user);

    if collection
        .find_one(doc! {"email": user.email.as_str()})
        .await
        .unwrap()
        .is_some()
    {
        return StatusCode::CONFLICT.into_response();
    }

    let user = User {
        id: None,
        name: user.name,
        email: user.email,
        password: hash(user.password, DEFAULT_COST).unwrap(),
    };
    collection.insert_one(user).await.unwrap();

    (StatusCode::OK, "account created").into_response()
}

pub async fn login(
    State(collection): State<Collection<User>>,
    Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return StatusCode::PRECONDITION_FAILED.into_response();
    }

    if let Some(user) = collection
        .find_one(doc! {"email": credentials.email.as_str()})
        .await
        .unwrap()
    {
        if verify(credentials.password, &user.password).unwrap() {
            let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
            let key = HS256Key::from_bytes(secret.as_bytes());
            let claims = Claims::create(Duration::from_mins(30)).with_subject(user.email);
            let token = key.authenticate(claims).unwrap();

            (StatusCode::OK, token).into_response()
        } else {
            StatusCode::UNAUTHORIZED.into_response()
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn auth_middleware(mut request: Request, next: Next) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token = bearer.token();

    let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
    let key = HS256Key::from_bytes(secret.as_bytes());
    let email = key
        .verify_token::<NoCustomClaims>(&token, None)
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .subject
        .unwrap();

    request.extensions_mut().insert(email);
    Ok(next.run(request).await)
}
