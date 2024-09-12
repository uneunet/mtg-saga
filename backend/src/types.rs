use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    #[serde(rename = "_id", skip_serializing)]
    #[ts(optional, rename = "_id", type = "ObjectId")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
}
