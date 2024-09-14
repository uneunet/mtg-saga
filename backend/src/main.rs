mod types;
mod auth;
mod routes;

use mongodb::{
    Client,
    Collection,
};
use std::env;
use crate::types::*;
use crate::routes::router;

use jwt_simple::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    let client = Client::with_uri_str(mongodb_uri).await.unwrap();

    let database = client.database("db");
    let collection: Collection<User> = database.collection("users");

    let router = router(collection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

