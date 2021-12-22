use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, web, Responder, HttpMessage};
use mongodb::Database;
use crate::models::NewGroupRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // rename to _id and use and document id in database
    id: String, // Same id as Discord guild id
    name: String,
    members: Vec<String>,        // id's of users that are members
    administrators: Vec<String>, // id's of users that are administrators
}

impl Group {
    pub fn new(
        id: String,
        name: String,
        creator: String,
    ) -> Group {
        let mut new_group = Group {
            id,
            name,
            members: Vec::<String>::new(),
            administrators: Vec::<String>::new(),
        };

        new_group.administrators.push(creator); // Make creator an admin

        new_group
    }
}

pub async fn create_group(
    database: web::Data<Database>,
    group: web::Json<NewGroupRequest>,
    req: HttpRequest,
) -> impl Responder {

    let jwt_token = req.cookie("auth-token")
}
