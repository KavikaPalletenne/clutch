use uuid::Uuid;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] // rename to _id and use and document id in database
    pub id: Option<ObjectId>,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Option<Vec<Tag>>, // Tags are optional
    pub resource_location: String, // URL to the data (stored on server or on something like AWS S3)
    pub created_at: chrono::NaiveDateTime,
    pub last_edited_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceForm {
    // No need for a document Id as MongoDB generates an Id for the document when you insert it
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Option<Vec<Tag>>, // Tags are optional
    pub resource_location: String, // URL to the data (stored on server or on something like AWS S3)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}