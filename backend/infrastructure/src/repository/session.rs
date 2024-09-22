use anyhow::Result;
use async_trait::async_trait;
use domain::{model::session::Session, repository::session_repository::SessionRepository};
use mongodb::{bson::doc, Client, Collection};

use std::env;

pub struct DBSessionRepositoryImpl {
    database: Collection<Session>,
}

impl DBSessionRepositoryImpl {
    pub async fn new() -> Result<Self> {
        let database_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");
        let collection: Collection<Session> = Client::with_uri_str(database_uri)
            .await?
            .database("db")
            .collection("sessions");

        Ok(Self {
            database: collection,
        })
    }
}

#[async_trait]
impl SessionRepository for DBSessionRepositoryImpl {
    async fn create(&self, session: Session) -> Result<()> {
        self.database.insert_one(session).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: String) -> Result<Option<Session>> {
        Ok(self.database.find_one(doc! { "token": token }).await?)
    }

    async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        Ok(self.database.find_one(doc! { "user_id": user_id }).await?)
    }

    async fn delete(&self, user_id: String) -> Result<()> {
        self.database
            .delete_one(doc! { "user_id": user_id })
            .await?;
        Ok(())
    }
}
