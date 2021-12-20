use crate::group::Group;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use crate::models::{NewUserRequest, UserExistsResponse};
use std::env;

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
#[get("/api/user/protected/create")]
pub async fn create_user(
    database: web::Data<Database>,
    web::Query(user_request): web::Query<NewUserRequest>,
) -> impl Responder {

    if user_request.secret != env::var("USER_SERVICE_SECRET").unwrap() {
        return HttpResponse::Unauthorized().body("This is a protected URI.");
    }

    let user = User::new(user_request.id, user_request.username, user_request.email);

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
#[get("/api/user/{id}")]
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
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&user).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid user id provided.")
}

// Read
#[get("/api/user/protected/userExists/{id}/{secret}")]
pub async fn user_exists(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let path = req.match_info();
    let user_id = path.get("id").unwrap().to_string();
    let secret = path.get("secret").unwrap().to_string();

    if secret != env::var("USER_SERVICE_SECRET").unwrap() {
        return HttpResponse::Unauthorized().body("This is a protected URI.");
    }

    let query = doc! {
        "_id": user_id,
    };

    let result: Option<User> = database
        .collection("users")
        .find_one(query, None)
        .await
        .expect("Could not fetch user with provided id");

    if let Some(_) = result {
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(
            &UserExistsResponse {
                exists: true,
            }
        ).unwrap());
    }

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(
        &UserExistsResponse {
            exists: false,
        }
    ).unwrap())
}

// Update username
#[post("/api/user/updateUsername/{id}/{username}")]
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

    HttpResponse::BadRequest().body("Could not update username.")
}

// Update email
#[post("/api/user/updateEmail/{id}/{email}")]
pub async fn update_email_by_user_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();
    let updated_email = req.match_info().get("username").unwrap().to_string();

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
            username: old_user.username,
            email: updated_email,
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
#[get("/api/user/delete/{id}")]
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
