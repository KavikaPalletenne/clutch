use crate::group::Group;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")] // rename to _id and use and document id in database
    id: String, // user id supplied from Discord etc.
    username: String,  // displayed as @<username>
    email: String,
    groups: Vec<Group>, // id of group that the user is a part of
}

impl User {
    pub fn new(id: String, username: String, email: String) -> User {
        User {
            id,
            username,
            email,
            groups: Vec::<Group>::new(),
        }
    }
}

/////////////////
// CRUD Functions
/////////////////

// Create
#[post("/api/protected/user/create")] // TODO: This function can only be called by the oauth2 user registration service. Maybe implement a secret (with every request) that only this and that know.
pub async fn create_user(
    database: web::Data<Database>,
    id: String,
    username: String,
    email: String,
) -> impl Responder {
    let user = User::new(id, username, email); // TODO: email is set using the email the user used for oauth2 for Google/Discord etc.

    let bson = bson::to_bson(&user).expect("Error converting struct to BSON");
    let document = bson.as_document().unwrap();

    let insert_result = database
        .collection("users")
        .insert_one(document.to_owned(), None)
        .await
        .expect("Error inserting document into collection");

    if insert_result.inserted_id.to_string().is_empty() {
        return HttpResponse::BadRequest().body("Error creating new user.");
    }

    HttpResponse::Ok().body("Successfully created user.")
}

// Read
#[get("/user/{id}")]
pub async fn get_user_by_id(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();

    let query = doc! {
        "_id": user_id,
    };

    let result: Option<User> = database
        .collection("users")
        .find_one(query, None)
        .await
        .expect("Could not fetch user with provided id");

    if let Some(user) = result {
        return HttpResponse::Ok().body(serde_json::to_string(&user).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid user id provided.")
}

// Update username
#[post("/user/updateUsername/{id}/{username}")]
pub async fn update_username_by_user_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();
    let updated_username = req.match_info().get("username").unwrap().to_string();

    let query = doc! {
        "_id": user_id,
    };

    let old_user_result: Option<User> = database
        .collection("users")
        .find_one(query, None)
        .await
        .expect("Could not update user.");

    if let Some(old_user) = old_user_result {
        let updated_user = User {
            id: old_user.id,
            username: updated_username,
            email: old_user.email,
            groups: old_user.groups,
        };

        let bson = bson::to_bson(&updated_user).expect("Error converting struct to BSON");
        let document = bson.as_document().unwrap();

        let insert_result = database
            .collection("users")
            .insert_one(document.to_owned(), None)
            .await
            .expect("Error inserting document into collection");

        if insert_result.inserted_id.to_string().is_empty() {
            return HttpResponse::BadRequest().body("Error updating username.");
        }
    }

    HttpResponse::BadRequest().body("Could not update user.")
}

// Delete
#[get("/user/delete/{id}")]
pub async fn delete_user_by_id(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();

    let filter = doc! {
        "_id": user_id,
    };

    let result = database
        .collection::<User>("users")
        .delete_one(filter, None)
        .await
        .expect("Error deleting user");

    if result.deleted_count == 0 {
        return HttpResponse::BadRequest().body("No such user exists.");
    }

    HttpResponse::Ok().body("Successfully deleted user.")
}
