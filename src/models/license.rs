use serde::{Serialize, Deserialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub license: String,
    pub wallet: String,
    pub community: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    pub activated: bool,
    pub created_at: i64,
}