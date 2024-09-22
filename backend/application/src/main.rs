mod routes;

use std::sync::Arc;
use anyhow::Result;
use axum::{routing::get, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse ;
use usecase::session::SessionService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let session = Arc::new(SessionService::new().await?);
    let routes = Router::new().route("/", get(test_handler)).with_state(session);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, routes).await?;

    Ok(())
}

async fn test_handler(State(session): State<Arc<SessionService>>) -> impl IntoResponse {
    session.create("yanknvim".into()).await.unwrap();
    StatusCode::OK.into_response()
}
