use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    #[serde(rename = "id", skip_serializing_if = "String::is_empty")]
    // rename to _id and use and document id in database
    pub id: String,
    pub user_id: String,  // owner
    pub group_id: String, // group it belongs to
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,         // Tags are optional
    pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
    pub last_edited_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReference {
    pub name: String, // This is the name the file will be stored under on the CDN
    pub size: i32,    // Size in bytes
}
