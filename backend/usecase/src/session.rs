use anyhow::Result;
use domain::{
    model::session::Session,
    repository::session_repository::SessionRepository,
};

pub struct SessionService {
    database: Box<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(database: Box<dyn SessionRepository>) -> Self {
        Self { database }
    }

    pub async fn create(&self, user_id: String) -> Result<()> {
        self.database.create(Session::new(user_id)).await
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<Session>> {
        self.database.find_by_id(user_id).await
    }
}