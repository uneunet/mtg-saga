mod middleware;
mod routes;

use anyhow::Result;
use axum::response::IntoResponse;
use usecase::UseCase;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let use_case = UseCase::new().await?;

    let routes = routes::routes(use_case);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, routes).await?;

    Ok(())
}
