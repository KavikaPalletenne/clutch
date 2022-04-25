use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use jsonwebtoken::DecodingKey;
use crate::middleware::authorize;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")] // rename to _id and use as document id in database
    pub id: String, // user id supplied from Discord etc.
    pub username: String,  // displayed as @<username>
    pub email: String,
    pub groups: Vec<String>, // id's of groups that the user is a part of
}

impl User {
    pub fn new(id: String, username: String, email: String) -> User {
        User {
            id,
            username,
            email,
            groups: Vec::<String>::new(),
        }
    }
}

/////////////////
// CRUD Functions
/////////////////

// Create
// #[post("/api/user/create")]
// pub async fn create_user_service(user_request: ProtoBuf<CreateUserRequest>, database: &Database) -> impl Responder { // Return true if success or false if failure
//     let user = User::new(user_request.0.id, user_request.0.username, user_request.0.email);
//
//     let bson = bson::to_bson(&user).expect("Error converting struct to BSON");
//     let document = bson.as_document().unwrap();
//
//     let insert_result = database
//         .collection("users")
//         .insert_one(document.to_owned(), None)
//         .await
//         .expect("Error inserting document into collection");
//
//     if insert_result.inserted_id.to_string().is_empty() {
//         return HttpResponse::BadRequest().protobuf(
//             CreateUserResponse {
//                 success: false
//             }
//         )
//     }
//
//     return HttpResponse::Ok().protobuf(
//         CreateUserResponse {
//             success: true
//         }
//     )
// }

// Read
#[get("/api/user/{id}")]
pub async fn get_user_by_id(database: web::Data<Database>, req: HttpRequest, decoding_key: web::Data<DecodingKey>) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req, decoding_key.get_ref()).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if authorized.user_id.unwrap().ne(&user_id) {
        return HttpResponse::Unauthorized().body("Logged in user does not have permission to view requested user.");
    }
    //////////////////////////////////////////////////////////////////////////


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
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&user).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid user id provided.")
}

#[get("/api/user/username/{id}")]
pub async fn get_username_by_id(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
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
            .append_header(("Content-Type", "application/json"))
            .body(format!(
                "{{
                    \"username\": \"{}\"
                }}", user.username
            ));
    }

    HttpResponse::BadRequest().body("Invalid user id provided.")
}


// Read
// #[get("/api/user/protected/userExists/{id}/{secret}")]
// pub async fn user_exists(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
//     let path = req.match_info();
//     let user_id = path.get("id").unwrap().to_string();
//     let secret = path.get("secret").unwrap().to_string();
//
//     if secret != env::var("USER_SERVICE_SECRET").unwrap() {
//         return HttpResponse::Unauthorized().body("This is a protected URI.");
//     }
//
//     let query = doc! {
//         "_id": user_id,
//     };
//
//     let result: Option<User> = database
//         .collection("users")
//         .find_one(query, None)
//         .await
//         .expect("Could not fetch user with provided id");
//
//     if let Some(_) = result {
//         return HttpResponse::Ok()
//             .append_header(("Content-Type", "application/json"))
//             .body(serde_json::to_string(
//                 &UserExistsResponse {
//                     exists: true,
//                 }
//             ).unwrap());
//     }
//
//     HttpResponse::Ok()
//         .append_header(("Content-Type", "application/json"))
//         .body(serde_json::to_string(
//             &UserExistsResponse {
//                 exists: false,
//             }
//         ).unwrap())
// }

// pub async fn user_exists_service(user_id: &String, database: &Database) -> bool {
//     let query = doc! {
//         "_id": user_id,
//     };
//
//     let result: Option<User> = database
//         .collection("users")
//         .find_one(query, None)
//         .await
//         .expect("Could not fetch user with provided id");
//
//     if let Some(_) = result {
//         return true;
//     }
//
//     false
// }

// Update username
#[post("/api/user/updateUsername/{id}/{username}")]
pub async fn update_username_by_user_id(
    database: web::Data<Database>,
    decoding_key: web::Data<DecodingKey>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();
    let updated_username = req.match_info().get("username").unwrap().to_string();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req, decoding_key.get_ref()).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("");
    }

    if authorized.user_id.unwrap().ne(&user_id) {
        return HttpResponse::Unauthorized().body("");
    }
    //////////////////////////////////////////////////////////////////////////

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

        return HttpResponse::Ok().body("Successfully updated username.");
    }

    HttpResponse::BadRequest().body("Could not update username.")
}

// Update email
#[post("/api/user/updateEmail/{id}/{email}")]
pub async fn update_email_by_user_id(
    database: web::Data<Database>,
    decoding_key: web::Data<DecodingKey>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();
    let updated_email = req.match_info().get("username").unwrap().to_string();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req, decoding_key.get_ref()).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("");
    }

    if authorized.user_id.unwrap().ne(&user_id) {
        return HttpResponse::Unauthorized().body("");
    }
    //////////////////////////////////////////////////////////////////////////


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

        return HttpResponse::Ok().body("Successfully updated email.");
    }

    HttpResponse::BadRequest().body("Could not update user.")
}

// Delete
#[get("/api/user/delete/{id}")]
pub async fn delete_user_by_id(database: web::Data<Database>, decoding_key: web::Data<DecodingKey>, req: HttpRequest) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap().to_string();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req, decoding_key.get_ref()).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("");
    }

    if authorized.user_id.unwrap().ne(&user_id) {
        return HttpResponse::Unauthorized().body("");
    }
    //////////////////////////////////////////////////////////////////////////

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

#[get("/api/user/get_user_groups/{user_id}")]
pub async fn get_user_groups(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let user_id = req.match_info().get("user_id").unwrap().to_string();

    let query = doc! {
        "_id": user_id,
    };

    let user = database
        .collection::<User>("users")
        .find_one(query, None)
        .await.expect("Error finding user in collection");

    if let Some(u) = user {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string::<Vec<String>>(&u.groups).unwrap());
    }

    HttpResponse::BadRequest()
        .append_header(("Content-Type", "text/plain"))
        .body("User does not exist.")
}
