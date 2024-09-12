mod types;

use axum::{
    Router,
    routing::get,
};
use mongodb::{
    Client,
    Database,
    Collection,
};
use std::env;
use crate::{
    types::*,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    let client = Client::with_uri_str(mongodb_uri).await.unwrap();

    let database = client.database("db");
    let collection: Collection<User> = database.collection("users");

    let router = Router::new()
        .route("/", get(root_handler))
        .with_state(collection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello!"
}
