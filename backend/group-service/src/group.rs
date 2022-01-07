use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, web, Responder, HttpMessage, HttpResponse, post, get};
use mongodb::Database;
use mongodb::bson::doc;
use crate::models::{NewGroupRequest, AuthorizeResponse, User, GroupUser};
use crate::middleware::authorize;
use actix_web::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id")]
    // rename to _id and use and document id in database
    id: String, // Same id as Discord guild id
    name: String,
    description: String,
    discord_link: String,
    members: Vec<GroupUser>,        // id's of users that are members
    administrators: Vec<GroupUser>, // id's of users that are administrators
}

impl Group {
    pub fn new(
        id: String,
        name: String,
        description: String,
        discord_link: String,
        creator: GroupUser,
    ) -> Group {
        let mut new_group = Group {
            id,
            name,
            description,
            discord_link,
            members: Vec::<GroupUser>::new(),
            administrators: Vec::<GroupUser>::new(),
        };

        new_group.administrators.push(creator); // Make creator an admin

        new_group
    }
}


// Create
#[post("/api/group/create")]
pub async fn create_group(
    database: web::Data<Database>,
    group: web::Json<NewGroupRequest>,
    req: HttpRequest,
) -> impl Responder {

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    // let authorized = authorize(req.clone()).await;
    //
    // if authorized.user_id.is_none() {
    //     return HttpResponse::Unauthorized().body("");
    // }
    //
    // if authorized.user_id.unwrap().ne(&group.creator_id) {
    //     return  HttpResponse::Unauthorized().body("");
    // }
    //////////////////////////////////////////////////////////////////////////
    // TODO: Check the current user is a part of the guild with provided id using Discord API [Optional](and check they are the owner).

    let user_query = doc! {
        "_id": group.creator_id.clone(),
    };

    let creator: User = database
        .collection("users")
        .find_one(user_query, None)
        .await
        .expect("Could not fetch user with provided id").unwrap();

    let group = Group::new(
        group.id.clone(),
        group.name.clone(),
        group.description.clone(),
        group.discord_link.clone(),
        GroupUser {
            id: creator.id,
            username: creator.username,
        },
    );

    let bson = bson::to_bson(&group).expect("Error converting struct to BSON");
    let document = bson.as_document().unwrap();

    let insert_result = database
        .collection("groups")
        .insert_one(document.to_owned(), None)
        .await
        .expect("Error inserting document into collection");

    if insert_result.inserted_id.to_string().is_empty() {
        return HttpResponse::BadRequest().body("Error creating new group.");
    }

    HttpResponse::Ok().body("Successfully created new group.")
}

#[get("/api/group/{id}")]
pub async fn get_group_by_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    // TODO: Maybe implement auth to check if the user can access this group using Discord API.
    let group_id = req.match_info().get("id").unwrap().to_string();

    let query = doc! {
        "_id": group_id,
    };

    let result: Option<Group> = database
        .collection("groups")
        .find_one(query, None)
        .await
        .expect("Could not fetch group with provided id");

    if let Some(group) = result {
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&group).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid group id provided.")
}

// TODO: Function to check whether user belongs to group
