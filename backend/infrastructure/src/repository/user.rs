use domain::{
    model::user::{User, UserInfo},
    repository::user_repository::UserRepository,
};
use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::{to_bson, doc},
    Collection,
};
use serde::{Serialize, Deserialize};

pub struct DBUserRepository {
    database: Collection<User>,
}

impl DBUserRepository {
    pub fn new(database: Collection<User>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl UserRepository for DBUserRepository {
    async fn create(&self, user: User) -> Result<()> {
        self.database.insert_one(user).await?;
        Ok(())
    }

    async fn find_by_id(&self, user_id: String) -> Result<Option<UserInfo>> {
        Ok(self.database
            .find_one(doc! { "user_id": user_id })
            .await?.map(|user| user.info))
    }

    async fn update_info(&self, user_id: String, info: UserInfo) -> Result<(), anyhow::Error> {
        let update = doc! { "$set": { "info": to_bson(&info).unwrap() } };
        self.database.update_one(doc! { "user_id": user_id }, update).await?;
        Ok(())
    }

    async fn delete(&self, user_id: String) -> Result<()> {
        self.database.delete_one(doc! {"user_id": user_id}).await?;
        Ok(())
    }
}
