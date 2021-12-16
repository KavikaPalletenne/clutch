use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // rename to _id and use and document id in database
    id: Option<ObjectId>,
    name: String,
    members: Vec<ObjectId>,        // id's of users that are members
    administrators: Vec<ObjectId>, // id's of users that are administrators
}
