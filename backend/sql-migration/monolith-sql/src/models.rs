use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sea_orm::{
    DeriveIntoActiveModel,
};


#[derive(Deserialize, Serialize)]
pub struct Resource {
    pub id: i64,
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
    // pub password: String, (DO NOT RETURN PASSWORD) // Hashed (argon2 hashing)
    pub discord_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FileReference {
    pub name: String,
    pub size: i32,
}

pub struct NewUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateUserForm {
    pub username: String,
    pub email: String,
}

pub struct Group {
    pub id: String,
    pub name: String,
    pub description: String,
    pub discord_id: String,
}

pub struct NewGroupForm {
    pub name: String,
    pub description: String,
    pub discord_id: String,
}
