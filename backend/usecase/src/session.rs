use anyhow::Result;
use domain::{model::session::Session, repository::session_repository::SessionRepository};
use infrastructure::repository::session::DBSessionRepositoryImpl;
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionUseCase {
    repository: Arc<dyn SessionRepository + Send + Sync>,
}

impl SessionUseCase {
    pub async fn new() -> Result<Self> {
        let repository = DBSessionRepositoryImpl::new().await?;
        Ok(Self {
            repository: Arc::new(repository),
        })
    }

    pub async fn create(&self, user_id: String) -> Result<Session> {
        let session = Session::new(user_id);
        self.repository.create(session.clone()).await?;
        Ok(session)
    }

    pub async fn find_by_token(&self, token: String) -> Result<Option<Session>> {
        self.repository.find_by_token(token).await
    }
    pub async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        self.repository.find_by_id(user_id).await
    }

    pub async fn delete(&self, user_id: String) -> Result<()> {
        self.repository.delete(user_id).await
    }
}
