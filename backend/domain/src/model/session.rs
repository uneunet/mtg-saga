use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: String, token: String) -> Self {
        Self {
            user_id,
            token,
            created_at: Utc::now(),
        }
    }
}
