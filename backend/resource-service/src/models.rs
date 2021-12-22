use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // rename to _id and use and document id in database
    pub id: Option<ObjectId>,
    pub user_id: String,  // owner
    pub group_id: String, // group it belongs to
    pub title: String,
    pub description: String,
    pub tags: Option<Vec<Tag>>,            // Tags are optional
    pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
    pub last_edited_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceForm {
    // No need for a document Id as MongoDB generates an Id for the document when you insert it
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub tags: Option<Vec<Tag>>,            // Tags are optional
    pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReference {
    pub id: Uuid, // This is the id the file will be stored under on the CDN
    pub title: String,
    pub size: i64, // Size in kilobytes
}
