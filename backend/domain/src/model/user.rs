use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub info: UserInfo,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
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
    pub fn new(info: UserInfo, password: String) -> Self {
        let password_hash = hash_password(password);

        Self {
            info,
            password_hash: password_hash,
            created_at: Utc::now(),
        }
    }

    pub fn verify_password(&self, password: String) -> bool {
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(&self.password_hash).unwrap();
        argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }
}

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    password_hash.to_string()
}
