use crate::{auth, types::*, users};
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar},
    headers::authorization::{Authorization, Bearer},
    TypedHeader,
};
use mongodb::Collection;

pub fn router(users: Collection<User>) -> Router {
    let api_router = Router::new()
        .nest("/auth", auth_router().with_state(users.clone()))
        .nest("/user", user_router().with_state(users));

    Router::new().nest("/api", api_router)
}

fn auth_router() -> Router<Collection<User>> {
    Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
}

fn user_router() -> Router<Collection<User>> {
    Router::new()
        .route("/:name", get(users::get_user_info_with_name))
        .route("/", get(users::get_user_info))
        .route("/delete", delete(users::delete_user))
        .layer(from_fn(auth::auth_middleware))
}
