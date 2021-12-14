use crate::models::{FileReference, Resource, ResourceForm, Tag};
use actix_web::web::resource;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use bson::Document;
use chrono::{NaiveDateTime, Utc};
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::{Cursor, Database};
use std::str::FromStr;
use tokio_stream::StreamExt;
use uuid::Uuid;
use std::borrow::Borrow;

// A resource is any document or link to a website.

impl Resource {
    pub fn new(
        id: Option<ObjectId>,
        user_id: Uuid,
        group_id: Uuid,
        title: String,
        description: String,
        tags: Option<Vec<Tag>>,
        files: Option<Vec<FileReference>>,
        last_edited_at: NaiveDateTime,
    ) -> Resource {
        Resource {
            id,
            user_id,
            group_id,
            title,
            description,
            tags,
            files,
            last_edited_at,
        }
    }
}

#[post("/resource/create")]
pub async fn create_resource(
    database: web::Data<Database>,
    resource: web::Json<ResourceForm>,
) -> impl Responder {
    // Check whether current user (JWT) is the same as resource user id.
    // TODO: Follow this for the CDN backend https://blog.logrocket.com/file-upload-and-download-in-rust/
    let id = Uuid::new_v4();
    let last_edited_at = Utc::now().naive_local();
    let resource = resource.into_inner();

    let id = Option::from(ObjectId::new());
    let resource = Resource::new(
        id,
        Uuid::from_str(resource.user_id.as_str()).unwrap(),
        Uuid::from_str(resource.group_id.as_str()).unwrap(),
        resource.title,
        resource.description,
        resource.tags,
        resource.files,
        last_edited_at,
    );

    let bson = bson::to_bson(&resource).expect("Error converting struct to BSON");
    let document = bson.as_document().unwrap();

    let insert_result = database
        .collection("notes")
        .insert_one(document.to_owned(), None)
        .await
        .expect("Error inserting document into collection");

    HttpResponse::Ok().body("Successfully created resource.")
}

#[get("/resource/get/{resource_id}")]
pub async fn fetch_resource_by_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let resource_id = ObjectId::from_str(req.match_info().get("resource_id").unwrap()).unwrap();

    let query = doc! {
        "_id": resource_id,
    };

    let result: Option<Resource> = database
        .collection("notes")
        .find_one(query, None)
        .await
        .expect("Could not fetch all documents for provided group id");

    if let Some(resource) = result {
        return HttpResponse::Ok().body(serde_json::to_string(&resource).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid resource id provided.")
}

#[get("/resource/getByGroupId/{group_id}")]
pub async fn fetch_resource_by_group_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let group_id = req.match_info().get("group_id").unwrap();

    let query = doc! {
        "group_id": group_id,
    };

    let mut cursor = database
        .collection("notes")
        .find(query, None)
        .await
        .expect("Could not fetch all documents for provided group id");

    let mut results: Vec<Resource> = Vec::new();
    while let Some(resource) = cursor.next().await {
        results.push(bson::from_document::<Resource>(resource.expect("Error")).expect("Error"))
    }

    if results.is_empty() {
        return HttpResponse::BadRequest().body("Group does not contain any resources.")
    }

    HttpResponse::Ok().body(serde_json::to_string::<Vec<Resource>>(&results).unwrap())
}

#[post("/resource/update/{resource_id}")]
pub async fn update_resource(
    database: web::Data<Database>,
    req: HttpRequest,
    resource_form: web::Json<ResourceForm>,
) -> impl Responder {
    let resource_id = ObjectId::from_str(req.match_info().get("resource_id").unwrap()).unwrap();

    let resource_form = resource_form.into_inner();
    let last_edited_at = Utc::now().naive_local();
    let query = doc! {
      "_id": resource_id,
    };

    let id = Option::from(resource_id);
    let resource = Resource::new(
        id,
        Uuid::from_str(resource_form.user_id.as_str()).unwrap(),
        Uuid ::from_str(resource_form.group_id.as_str()).unwrap(),
        resource_form.title.clone(),
        resource_form.description.clone(),
        resource_form.tags.clone(),
        resource_form.files.clone(),
        last_edited_at,
    );

    let bson_document = bson::to_bson(&resource_form).expect("Error converting struct to BSON");
    let doc = bson_document.as_document().unwrap().clone();

    let result = database
        .collection::<Resource>("notes")
        .replace_one(query,  resource, None)
        .await
        .expect("Error updating document");

    if result.modified_count == 0 {
        return HttpResponse::BadRequest().body("No such resource exists.");
    }

    HttpResponse::Ok().body("Successfully updated resource.")
}

#[get("/resource/delete/{resource_id}")]
pub async fn delete_resource(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let resource_id = ObjectId::from_str(req.match_info().get("resource_id").unwrap()).unwrap();

    let filter = doc! {
        "_id": resource_id,
    };

    let result = database
        .collection::<Resource>("notes")
        .delete_one(filter, None)
        .await
        .expect("Error deleting document");

    if result.deleted_count == 0 {
        return HttpResponse::BadRequest().body("No such resource exists.");
    }

    HttpResponse::Ok().body("Successfully deleted resource.")
}
