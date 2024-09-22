use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub info: UserInfo,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserInfo {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 32))]
    pub user_id: String,
}

impl User {
    pub fn new(info: UserInfo, password_hash: String) -> Self {
        Self {
            info,
            password_hash,
        }
    }
}
