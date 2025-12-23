use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub author: String,
    /// How many copies are currently available in stock
    pub stock: i32,
}
