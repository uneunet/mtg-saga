use anyhow::{Context, Result};
use domain::{
    model::user::{User, UserInfo},
    repository::user_repository::UserRepository,
};
use infrastructure::repository::user::DBUserRepositoryImpl;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserUseCase {
    pub async fn new() -> Result<Self> {
        let repository = DBUserRepositoryImpl::new().await?;
        Ok(Self {
            repository: Arc::new(repository),
        })
    }

    pub async fn create(
        &self,
        name: String,
        email: String,
        user_id: String,
        password: String,
    ) -> Result<User> {
        let info = UserInfo {
            name,
            email,
            user_id,
        };
        let user = User::new(info, password);
        self.repository.create(user.clone()).await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<User>> {
        self.repository.find_by_id(user_id).await
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<User>> {
        self.repository.find_by_email(email).await
    }
    pub async fn verify_password(&self, user_id: String, password: String) -> Result<Option<bool>> {
        Ok(self
            .repository
            .find_by_id(user_id)
            .await?
            .map(|user| user.verify_password(password)))
    }

    pub async fn delete(&self, user_id: String) -> Result<()> {
        self.repository.delete(user_id).await
    }
}
