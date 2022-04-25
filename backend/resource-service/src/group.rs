use actix_web::web;
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id")]
    // rename to _id and use and document id in database
    pub id: String, // Same id as Discord guild id
    pub name: String,
    pub description: String,
    pub discord_link: String,
    pub members: Vec<String>,        // id's of users that are members
    pub administrators: Vec<String>, // id's of users that are administrators
}

pub async fn check_user_in_group(user_id: String, group_id: String, database: &web::Data<Database>) -> bool {
    let query = doc! {
        "_id": group_id,
    };

    let result: Option<Group> = database
        .collection("groups")
        .find_one(query, None)
        .await
        .expect("Could not fetch group with provided id");

    if let Some(group) = result {
        if group.members.contains(&user_id) || group.administrators.contains(&user_id) {
            return true;
        }
    }
    false
}
