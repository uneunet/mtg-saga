use domain::{
    model::session::Session,
    repository::session_repository::SessionRepository,
};
use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::{to_bson, doc},
    Collection,
};

pub struct DBSessionRepository {
    database: Collection<Session>,
}

impl DBSessionRepository {
    pub fn new(database: Collection<Session>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl SessionRepository for DBSessionRepository {
    async fn create(&self, session: Session) -> Result<()> {
        self.database.insert_one(session).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: String) -> Result<Option<Session>> {
        Ok(self.database
            .find_one(doc! { "token": token })
            .await?)
    }

    async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        Ok(self.database
            .find_one(doc! { "user_id": user_id })
            .await?)

    }

    async fn delete(&self, user_id: String) -> Result<()> {
        self.database.delete_one(doc! { "user_id": user_id }).await?;
        Ok(())
    }
}
