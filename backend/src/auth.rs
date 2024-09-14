use crate::types::*;
use axum::{
    extract::{Form, State, FromRequestParts},
    response::{IntoResponse, Response},
    http::StatusCode,
    extract::Request,
    middleware::{self, Next},
    RequestExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{
        oid::ObjectId,
        doc,
    },
    Collection,
};
use jwt_simple::prelude::*;
use std::env;

pub async fn sign_up(
    State(collection): State<Collection<User>>,
    Form(credentials): Form<Credentials>
) -> impl IntoResponse {
    if collection.find_one(doc! {"email": credentials.email.as_str()}).await.unwrap().is_some() {
        return (
            StatusCode::CONFLICT,
            "email is already registered",
        ).into_response()
    }
    if credentials.password.is_empty() {
        return (
            StatusCode::PRECONDITION_FAILED,
            "password is empty",
        ).into_response()
    }

    let user = User {
        id: None,
        email: credentials.email,
        password: credentials.password,
    };
    println!("Creating User...");
    collection.insert_one(user.clone()).await.unwrap();

    (StatusCode::OK, "account created").into_response()
}

pub async fn login(
    State(collection): State<Collection<User>>,
    Form(credentials): Form<Credentials>
) -> impl IntoResponse {
    if let Some(user) = collection.find_one(doc! {"email": credentials.email.as_str()}).await.unwrap() {
        if user.password == credentials.password {
            let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
            let key = HS256Key::from_bytes(secret.as_bytes());
            let claims = Claims::create(Duration::from_days(3))
                .with_subject(user.email);
            let token = key.authenticate(claims).unwrap();

            (StatusCode::OK, token).into_response()
        } else {
            (StatusCode::UNAUTHORIZED, "password is incorrect").into_response()
        }
    } else {
        (StatusCode::NOT_FOUND, "email not found").into_response()
    }
}

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token = bearer.token();

    let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
    let key = HS256Key::from_bytes(secret.as_bytes());
    key.verify_token::<NoCustomClaims>(&token, None)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}
