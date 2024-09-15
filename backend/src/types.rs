use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    #[serde(rename = "_id", skip_serializing)]
    #[ts(optional, rename = "_id", type = "ObjectId")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
