use anyhow::Result;
use domain::{model::session::Session, repository::session_repository::SessionRepository};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub struct SessionService {
    database: Box<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(database: Box<dyn SessionRepository>) -> Self {
        Self { database }
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        self.database.find_by_id(user_id).await
    }

    pub async fn create(&self, user_id: String) -> Result<()> {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let mut token = [0u8; 16];
        rng.fill_bytes(&mut token);
        let token = token
            .iter()
            .map(|s| format!("{:02X}", s))
            .collect::<String>();

        self.database.create(Session::new(user_id, token)).await
    }
}
