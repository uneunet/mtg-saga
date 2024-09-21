use crate::{auth, types::*, users};
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
    Router,
};

pub fn router(states: DBStates) -> Router {
    let api_router = Router::new()
        .nest("/auth", auth_router().with_state(states.clone()))
        .nest("/user", user_router(states.clone()).with_state(states));

    Router::new().nest("/api", api_router)
}

fn auth_router() -> Router<DBStates> {
    Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
}

fn user_router(states: DBStates) -> Router<DBStates> {
    Router::new()
        .route("/:name", get(users::get_user_info_with_name))
        .route("/", get(users::get_user_info))
        .route("/delete", delete(users::delete_user))
        .layer(from_fn_with_state(states, auth::auth_middleware))
}
