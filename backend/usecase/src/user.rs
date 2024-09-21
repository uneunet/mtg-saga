use anyhow::Result;
use domain::{
    model::user::{User, UserInfo},
    repository::user_repository::UserRepository,
};

pub struct UserService {
    repository: Box<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, user: User) -> Result<()> {
        self.repository.create(user).await
    }

    pub async fn delete(&self, user_id: String) -> Result<()> {
        self.repository.delete(user_id).await
    }
}
