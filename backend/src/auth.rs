use crate::types::*;
use axum::{
    extract::Request,
    extract::{Json, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use bcrypt::{hash, verify, DEFAULT_COST};
use jwt_simple::prelude::*;
use mongodb::{bson::doc, Collection};
use std::env;

pub async fn sign_up(
    State(users): State<Collection<User>>,
    Json(user): Json<User>,
) -> axum::response::Result<Response, StatusCode> {
    if user.name.is_empty() || user.email.is_empty() || user.password.is_empty() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }
    println!("{:?}", user);

    if users
        .find_one(doc! {"email": user.email.as_str()})
        .await
        .unwrap()
        .is_some()
    {
        return Err(StatusCode::CONFLICT);
    }

    let user = User {
        id: None,
        name: user.name,
        email: user.email,
        password: hash(user.password, DEFAULT_COST).unwrap(),
    };
    users.insert_one(user).await.unwrap();

    Ok(StatusCode::OK.into_response())
}

pub async fn login(
    State(users): State<Collection<User>>,
    Json(credentials): Json<Credentials>,
) -> axum::response::Result<Response, StatusCode> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }

    if let Some(user) = users
        .find_one(doc! {"email": credentials.email.as_str()})
        .await
        .unwrap()
    {
        if verify(credentials.password, &user.password).unwrap() {
            let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
            let key = HS256Key::from_bytes(secret.as_bytes());
            let claims = Claims::create(Duration::from_mins(30)).with_subject(user.email);
            let token = key.authenticate(claims).unwrap();

            Ok(token.into_response())
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response, StatusCode> {
    if let Some(token) = jar.get("jwt") {
        let secret = env::var("KEY_SECRET").expect("KEY_SECRET not found");
        let key = HS256Key::from_bytes(secret.as_bytes());
        let email = key
            .verify_token::<NoCustomClaims>(token.value(), None)
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .subject
            .unwrap();

        request.extensions_mut().insert(email);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
