use mongodb::{
    Collection,
    bson::{
        oid::ObjectId,
        DateTime,
    },
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub info: UserInfo,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserInfo {
    #[validate(length(min = 4, max = 32))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Credential {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 7, max = 64))]
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct FormUser {
    #[validate(length(min = 4, max = 32))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 7, max = 64))]
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Session {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub user: ObjectId,
    pub token: String,
    pub created_at: DateTime,
}


#[derive(Clone, Debug)]
pub struct DBStates {
    pub users: Collection<User>,
    pub sessions: Collection<Session>,
}
