use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

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
        let token_str = token
            .iter()
            .map(|s| format!("{:02X}", s))
            .collect::<String>();

        Self {
            user_id,
            token: token_str,
            created_at: Utc::now(),
        }
    }
}

