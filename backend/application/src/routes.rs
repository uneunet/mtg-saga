use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use usecase::UseCase;

mod auth;
mod user;

pub fn routes(use_case: UseCase) -> Router {
    let api_router = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth::routes(use_case.clone()))
        .nest("/user", user::routes(use_case));

    Router::new().nest("/api", api_router)
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
