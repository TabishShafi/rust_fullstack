use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,  // MongoDB ID, automatically generated
    pub name: String,          // Patient name
    pub age: u32,              // Patient age
    pub condition: String,     // Patient’s medical condition
}