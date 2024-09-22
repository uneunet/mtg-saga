mod routes;


use axum::{
    Router,
    routing::get,
};

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/", get(|_| "hello!"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000");
    axum::serve(listener, routes).await.unwrap();
}
