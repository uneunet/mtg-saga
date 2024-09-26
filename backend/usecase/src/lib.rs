pub mod session;
pub mod user;

use crate::session::SessionUseCase;
use crate::user::UserUseCase;
use anyhow::Result;

#[derive(Clone)]
pub struct UseCase {
    pub session: SessionUseCase,
    pub user: UserUseCase,
}

impl UseCase {
    pub async fn new() -> Result<Self> {
        let session = SessionUseCase::new().await?;
        let user = UserUseCase::new().await?;

        Ok(Self { session, user })
    }
}
