use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, web, Responder, HttpResponse, post, get};
use mongodb::Database;
use mongodb::bson::doc;
use crate::models::{NewGroupRequest, User, GroupUser};
use crate::middleware::{authorize, find_and_remove_string_from_vector, find_and_remove_user_from_vector};

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

impl Group {
    pub fn new(
        id: String,
        name: String,
        description: String,
        discord_link: String,
        creator: String,
    ) -> Group {
        let mut new_group = Group {
            id,
            name,
            description,
            discord_link,
            members: Vec::<String>::new(),
            administrators: Vec::<String>::new(),
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
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if authorized.user_id.unwrap().ne(&group.creator_id) {
        return  HttpResponse::Unauthorized().body("Incorrect user id supplied.");
    }
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
        creator.id,
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

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if !check_user_in_group(authorized.user_id.clone().unwrap(), group_id.to_string().clone(), &database).await {
        return HttpResponse::Unauthorized().body("Logged in user not in group.");
    }
    //////////////////////////////////////////////////////////////////////////

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

#[get("/api/group/join/{id}")]
pub async fn join_group(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let group_id = req.match_info().get("id").unwrap().to_string();
    let user_response = authorize(&req).await;

    if let Some(id) = user_response.user_id {
        let query = doc! {
            "_id": group_id.clone(),
        };

        let result: Option<Group> = database
            .collection("groups")
            .find_one(query.clone(), None)
            .await
            .expect("Could not fetch group with provided id");

        if let Some(mut group) = result {
            // TODO: Add the logged in user to the group if they don't already exist.
            if group.members.contains(
                &id.clone()
            ) ||
            group.administrators.contains(
                &id.clone()
            ) {
                return HttpResponse::BadRequest().body("Already joined group.")
            }
            
            group.members.push(
                id.clone()
            );

            let update_result = database
                .collection::<Group>("groups")
                .replace_one(query.clone(), group, None)
                // .insert_one(document.to_owned(), None)
                .await
                .expect("Error updating document in collection");

            if update_result.modified_count == 0 {
                return HttpResponse::BadRequest()
                    .header("Content-Type", "text/plain")
                    .body("Error joining group.");
            }

            // Add group to user's record
            let user_query = doc! {
                "_id": id.clone(),
            };

            let mut user: User = database
                .collection("users")
                .find_one(user_query.clone(), None)
                .await.expect("Error finding user").unwrap();

            user.groups.push(group_id.clone());

            let user_update_response = database
                .collection::<User>("users")
                .replace_one(user_query, user, None)
                .await.expect("Error updating user");

            if user_update_response.modified_count == 0 {
                return HttpResponse::BadRequest()
                    .header("Content-Type", "text/plain")
                    .body("Error joining group.");
            }

            return HttpResponse::Ok()
                .header("Content-Type", "text/plain")
                .body("Successfully joined group")
        }
    }
    HttpResponse::Unauthorized()
        .header("Content-Type", "text/plain")
        .body("Not logged in.")
}

#[get("/api/group/leave/{id}")]
pub async fn leave_group(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let group_id = req.match_info().get("id").unwrap().to_string();
    let user_response = authorize(&req).await;

    if let Some(id) = user_response.user_id {
        let query = doc! {
            "_id": group_id.clone(),
        };

        let result: Option<Group> = database
            .collection("groups")
            .find_one(query.clone(), None)
            .await
            .expect("Could not fetch group with provided id");

        if let Some(mut group) = result {
            if group.members.contains(
                &id.clone()
            ) {
                find_and_remove_string_from_vector(
                    &mut group.members,
                    id.clone(),
                );
            }

            if group.administrators.contains(
                &id.clone()
            ) {
                find_and_remove_string_from_vector(
                    &mut group.administrators,
                    id.clone()
                );
            }

            let update_result = database
                .collection::<Group>("groups")
                .replace_one(query.clone(), group, None)
                // .insert_one(document.to_owned(), None)
                .await
                .expect("Error updating document in collection");

            if update_result.modified_count == 0 {
                return HttpResponse::BadRequest()
                    .header("Content-Type", "text/plain")
                    .body("Error leaving group.");
            }

            // Remove group from user's record
            let user_query = doc! {
                "_id": id.clone(),
            };

            let mut user: User = database
                .collection("users")
                .find_one(user_query.clone(), None)
                .await.expect("Error finding user").unwrap();

            find_and_remove_string_from_vector(
                &mut user.groups,
                group_id.clone()
            );

            let user_update_response = database
                .collection::<User>("users")
                .replace_one(user_query, user, None)
                .await.expect("Error updating user");

            if user_update_response.modified_count == 0 {
                return HttpResponse::BadRequest()
                    .header("Content-Type", "text/plain")
                    .body("Error leaving group.");
            }


            return HttpResponse::Ok()
                .header("Content-Type", "text/plain")
                .body("Successfully left group.");
        }
        return HttpResponse::BadRequest()
            .header("Content-Type", "text/plain")
            .body("Could not find group.");
    }
    HttpResponse::Unauthorized()
        .header("Content-Type", "text/plain")
        .body("Not logged in.")
}

// TODO: Function to check whether user belongs to group
/// Returns true if user is in group, otherwise returns false
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

/// Returns true if user is admin of group, otherwise returns false
pub async fn check_user_is_group_admin(user_id: String, group_id: String, database: &web::Data<Database>) -> bool {
    let query = doc! {
        "_id": group_id,
    };

    let result: Option<Group> = database
        .collection("groups")
        .find_one(query, None)
        .await
        .expect("Could not fetch group with provided id");

    if let Some(group) = result {
        if group.administrators.contains(&user_id) {
            return true;
        }
    }
    false
}
