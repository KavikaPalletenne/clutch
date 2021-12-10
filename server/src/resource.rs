use actix_web::{HttpResponse, Responder, web, get, post, HttpRequest};
use bson::oid::ObjectId;
use chrono::{NaiveDateTime, Utc};
use mongodb::{Database};
use uuid::Uuid;
use crate::models::{Resource, ResourceForm, Tag};

// A resource is any document or link to a website.

impl Resource {

    pub fn new(id: Option<ObjectId>, user_id: Uuid, title: String, description: String, tags: Option<Vec<Tag>>,
        resource_location: String, created_at: NaiveDateTime, last_edited_at: NaiveDateTime,
    ) -> Resource {
        Resource {
            id,
            user_id,
            title,
            description,
            tags,
            resource_location,
            created_at,
            last_edited_at,
        }
    }
}

#[post("/resource/create")]
pub async fn create_resource(database: web::Data<Database>, resource: web::Form<ResourceForm>) -> impl Responder {
    // Check whether current user (JWT) is the same as resource user id.
    // TODO: Use MongoDB's GridFS to upload any PDF file straight to the database.
    let id = Uuid::new_v4();
    let created_at = Utc::now().naive_local();
    let last_edited_at = Utc::now().naive_local();
    let resource = resource.into_inner();

    let id = Option::from(ObjectId::new());
    let resource = Resource::new(
        id,
        resource.user_id,
        resource.title,
        resource.description,
        resource.tags,
        resource.resource_location,
        created_at,
        last_edited_at,
    );

    let bson = bson::to_bson(&resource).expect("Error converting struct to BSON"); // TODO: How to convert struct to a BSON document
    let document = bson.as_document().unwrap();

    let insert_result = database.collection("notes").insert_one(document.to_owned(), None).await.expect("Error inserting document into collection");

    HttpResponse::Ok().body("Successfully created resource.")
}

#[post("/resource/update/{resource_id}")]
pub async fn update_resource(database: web::Data<Database>, req: HttpRequest, resource: web::Form<ResourceForm>) -> impl Responder {

    let resource_id = req.match_info().get("resource_id");

    if let Some(id) = resource_id {
        let last_edited_at = Utc::now().naive_local();

        //let resource = database.collection("notes").find_one_and_update(); // TODO: Find how to do this

        return HttpResponse::Ok().body("Successfully updated resource.");
    }

    HttpResponse::BadRequest().body("No resource id provided.")
}

#[post("/resource/update2/{resource_id}")]
pub async fn update_resource2(database: web::Data<Database>, req: HttpRequest) -> impl Responder {

    let resource_id = req.match_info().get("resource_id");

    if let Some(id) = resource_id {
        let last_edited_at = Utc::now().naive_local();

        //let resource = database.collection("notes").find_one_and_update(); // TODO: Find how to do this

        return HttpResponse::Ok().body("Successfully updated resource.");
    }

    HttpResponse::BadRequest().body("No resource id provided.")
}