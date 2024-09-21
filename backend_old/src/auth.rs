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
use mongodb::bson::{
    DateTime,
    doc,
};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use validator::Validate;

pub async fn sign_up(
    State(states): State<DBStates>,
    Json(user): Json<FormUser>,
) -> axum::response::Result<Response, StatusCode> {
    let users = states.users;

    if user.validate().is_err() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }

    if users
        .find_one(doc! {"email": user.email.as_str()})
        .await
        .unwrap()
        .is_some()
    {
        return Err(StatusCode::CONFLICT);
    }

    let user_info = UserInfo {
        name: user.name,
        email: user.email,
    };

    let user = User {
        id: None,
        info: user_info,
        password_hash: hash(user.password, DEFAULT_COST).unwrap(),
    };
    users.insert_one(user).await.unwrap();

    Ok(StatusCode::OK.into_response())
}

pub async fn login(
    State(states): State<DBStates>,
    Json(credential): Json<Credential>,
) -> axum::response::Result<Response, StatusCode> {
    let users = states.users;
    let sessions = states.sessions;

    if credential.validate().is_err() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }

    if let Some(user) = users
        .find_one(doc! {"info.email": credential.email.as_str()})
        .await
        .unwrap()
    {
        if verify(credential.password, &user.password_hash).unwrap() {
            let mut rng = ChaCha20Rng::from_entropy();
            let mut token = [0u8; 16];
            rng.fill_bytes(&mut token);
            let token_str = token.iter().map(|s| format!("{:02X}", s)).collect::<String>();

            let session = Session {
                id: None,
                user: user.id.unwrap(),
                token: token_str.clone(),
                created_at: DateTime::now(),
            };
            sessions
                .insert_one(session)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(token_str.into_response())
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_middleware(
    State(states): State<DBStates>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response, StatusCode> {
    let sessions = states.sessions;

    if let Some(token) = jar.get("token") {
        let user = sessions
            .find_one(doc! { "token": token.value() })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .unwrap()
            .user;

        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
