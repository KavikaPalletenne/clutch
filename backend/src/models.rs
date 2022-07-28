use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Resource {
    pub id: String,
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileReference>>,
    pub last_edited_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceForm {
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileReference>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SearchResource {
    pub id: String,
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileReference>>,
    pub last_edited_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct DbResource {
    pub id: String,
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub last_edited_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    // pub password: String, (DO NOT RETURN PASSWORD) // Hashed (argon2 hashing)
    pub discord_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub discord_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CreateUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FileReference {
    pub name: String,
    pub size: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreatedResourceResponse {
    pub resource_id: i64,
    pub group_id: String,
    pub file_put_urls: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct NewUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct DiscordLinkForm {
    pub email: String,
    pub password: String,
    pub discord_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUserForm {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: String,
    pub discord_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GroupResponse {
    #[serde(rename = "_id")]
    // rename to _id and use and document id in database
    pub id: String, // Same id as Discord guild id
    pub name: String,
    pub description: String,
    pub discord_link: String,
    pub members: Vec<String>,        // id's of users that are members
    pub administrators: Vec<String>, // id's of users that are administrators
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NewGroupForm {
    pub name: String,
    pub description: String,
    pub discord_id: String,
    pub private: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenQuery {
    pub token: String,
}
