use crate::model::user::{User, UserInfo};
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<()>;
    async fn find_by_id(&self, user_id: String) -> Result<Option<UserInfo>>;
    async fn update_info(&self, user_id: String, info: UserInfo) -> Result<()>;
    async fn delete(&self, user_id: String) -> Result<()>;
}
