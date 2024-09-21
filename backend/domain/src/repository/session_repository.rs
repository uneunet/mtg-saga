use crate::model::session::Session;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait SessionRepository {
    async fn create(&self, session: Session) -> Result<()>;
    async fn find_by_token(&self, token: String) -> Result<Option<Session>>;
    async fn find_by_id(&self, user_id: String) -> Result<Option<Session>>;
    async fn delete(&self, user_id: String) -> Result<()>;
}
