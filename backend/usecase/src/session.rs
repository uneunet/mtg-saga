use anyhow::Result;
use domain::{model::session::Session, repository::session_repository::SessionRepository};
use infrastructure::repository::session::DBSessionRepositoryImpl;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub struct SessionService {
    repository: Box<dyn SessionRepository + Send + Sync>,
}

impl SessionService {
    pub async fn new() -> Result<Self> {
        let repository = DBSessionRepositoryImpl::new().await?;
        Ok(Self { repository: Box::new(repository) })
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        self.repository.find_by_id(user_id).await
    }

    pub async fn create(&self, user_id: String) -> Result<()> {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let mut token = [0u8; 16];
        rng.fill_bytes(&mut token);
        let token = token
            .iter()
            .map(|s| format!("{:02X}", s))
            .collect::<String>();

        self.repository.create(Session::new(user_id, token)).await
    }
}
