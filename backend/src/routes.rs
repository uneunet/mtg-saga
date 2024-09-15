use crate::{auth, types::*, users};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use mongodb::Collection;

pub fn router(collection: Collection<User>) -> Router {
    let api_router = Router::new()
        .nest("/auth", auth_router().with_state(collection.clone()))
        .nest("/user", user_router().with_state(collection));

    Router::new().nest("/api", api_router)
}

fn auth_router() -> Router<Collection<User>> {
    Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
}

fn user_router() -> Router<Collection<User>> {
    Router::new()
        .route("/", get(users::get_user_info))
        .layer(from_fn(auth::auth_middleware))
}
