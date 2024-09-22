use anyhow::Result;
use domain::{
    model::user::{User, UserInfo},
    repository::user_repository::UserRepository,
};
use infrastructure::repository::user::DBUserRepositoryImpl;

pub struct UserService {
    repository: Box<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub async fn new() -> Result<Self> {
        let repository = DBUserRepositoryImpl::new().await?;
        Ok(Self {
            repository: Box::new(repository),
        })
    }

    pub async fn create(&self, user: User) -> Result<()> {
        self.repository.create(user).await
    }

    pub async fn delete(&self, user_id: String) -> Result<()> {
        self.repository.delete(user_id).await
    }
}
