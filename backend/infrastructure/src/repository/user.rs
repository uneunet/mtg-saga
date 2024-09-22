use anyhow::Result;
use async_trait::async_trait;
use domain::{
    model::user::{User, UserInfo},
    repository::user_repository::UserRepository,
};
use mongodb::{
    bson::{doc, to_bson},
    Collection,
};
use serde::{Deserialize, Serialize};

pub struct DBUserRepositoryImpl {
    database: Collection<User>,
}

impl DBUserRepositoryImpl {
    pub fn new(database: Collection<User>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl UserRepository for DBUserRepositoryImpl {
    async fn create(&self, user: User) -> Result<()> {
        self.database.insert_one(user).await?;
        Ok(())
    }

    async fn find_by_id(&self, user_id: String) -> Result<Option<UserInfo>> {
        Ok(self
            .database
            .find_one(doc! { "user_id": user_id })
            .await?
            .map(|user| user.info))
    }

    async fn update_info(&self, user_id: String, info: UserInfo) -> Result<()> {
        let update = doc! { "$set": { "info": to_bson(&info)? } };
        self.database
            .update_one(doc! { "user_id": user_id }, update)
            .await?;
        Ok(())
    }

    async fn delete(&self, user_id: String) -> Result<()> {
        self.database.delete_one(doc! {"user_id": user_id}).await?;
        Ok(())
    }
}
