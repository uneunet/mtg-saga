mod auth;
mod routes;
mod types;
mod users;

use crate::routes::router;
use crate::types::*;
use mongodb::{Client, Collection, IndexModel, options::IndexOptions, bson::doc};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenvy::dotenv().expect(".env file not found");

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    let client = Client::with_uri_str(mongodb_uri).await.unwrap();

    let database = client.database("db");
    let users: Collection<User> = database.collection("users");
    let sessions: Collection<Session> = database.collection("sessions");

    let opts = IndexOptions::builder()
        .expire_after(Duration::new(60 * 60 * 24 * 7, 0)) // 7 days to expire
        .build();

    let index = IndexModel::builder()
        .keys(doc! {"created_at": 1})
        .options(opts)
        .build();

    sessions.create_index(index).await.unwrap();

    let router = router(DBStates { users, sessions });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
