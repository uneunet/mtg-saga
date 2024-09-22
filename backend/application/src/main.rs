mod routes;

use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let routes = Router::new().route("/", get(|| async { "hello!"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, routes).await?;

    Ok(())
}
