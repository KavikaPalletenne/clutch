use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGroupRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub discord_link: String,
    pub creator_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeResponse {
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub owner: bool,
    pub permission: String,
    pub features: Vec::<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildResponse {
    pub guilds: Vec::<PartialGuild>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")] // rename to _id and use as document id in database
    pub id: String, // user id supplied from Discord etc.
    pub username: String,  // displayed as @<username>
    pub email: String,
    pub groups: Vec<String>, // id's of groups that the user is a part of
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupUser {
    pub id: String,
    pub username: String,
}
