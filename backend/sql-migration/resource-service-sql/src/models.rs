use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sea_orm::{
    DeriveIntoActiveModel,
};


#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct ResourceForm {
    pub user_id: String,
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileReference>>,
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

pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String, // Hashed (argon2 hashing)
    pub discord_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FileReference {
    pub name: String,
    pub size: i32,
}

pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}
