use chrono::{DateTime, Utc};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: String) -> Self {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let mut token = [0u8; 16];
        rng.fill_bytes(&mut token);
        let token = token
            .iter()
            .map(|s| format!("{:02X}", s))
            .collect::<String>();
        Self {
            user_id,
            token,
            created_at: Utc::now(),
        }
    }
}
