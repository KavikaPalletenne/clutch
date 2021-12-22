use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id")]
    // rename to _id and use and document id in database
    id: String, // Same id as Discord guild id
    name: String,
    members: Vec<String>,        // id's of users that are members
    administrators: Vec<String>, // id's of users that are administrators
}
