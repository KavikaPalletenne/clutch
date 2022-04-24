use crate::models::{CreatedResourceResponse, FileReference, Resource, ResourceForm};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use chrono::{NaiveDateTime, Utc};
use mongodb::bson::doc;
use mongodb::Database;
use std::str::FromStr;
use meilisearch_sdk::document::Document;
use meilisearch_sdk::indexes::Index;
use nanoid::nanoid;
use s3::Bucket;
use tokio_stream::StreamExt;
use crate::group::check_user_in_group; // TODO: Create endpoint in grpc
use crate::middleware::authorize;

// A resource is any document or link to a website.
impl Resource {
    pub fn new(
        id: String,
        user_id: String,
        group_id: String,
        title: String,
        description: String,
        subject: String,
        tags: Option<Vec<String>>,
        files: Option<Vec<FileReference>>,
        last_edited_at: NaiveDateTime,
    ) -> Resource {
        Resource {
            id,
            user_id,
            group_id,
            title,
            description,
            subject,
            tags,
            files,
            last_edited_at,
        }
    }
}

impl Document for Resource {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType { &self.id }
}

/////////////////
// CRUD Functions
/////////////////

#[post("/api/resource/create")]
pub async fn create_resource(
    req: HttpRequest,
    database: web::Data<Database>,
    bucket: web::Data<Bucket>,
    index: web::Data<Index>,
    resource: web::Json<ResourceForm>,
) -> impl Responder {

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if !check_user_in_group(authorized.user_id.clone().unwrap(), resource.group_id.clone(), &database).await {
        return HttpResponse::Unauthorized().body("Logged in user not in group.");
    }
    //////////////////////////////////////////////////////////////////////////

    let last_edited_at = Utc::now().naive_local();
    let resource_form = resource.into_inner();
    let files = resource_form.files.clone();

    let group_id = resource_form.group_id.clone();

    let id: String = nanoid!();
    let resource = Resource::new(
        id.clone(),
        authorized.user_id.unwrap(),
        resource_form.group_id,
        resource_form.title,
        resource_form.description,
        resource_form.subject,
        resource_form.tags,
        files.clone(),
        last_edited_at,
    );

    let bson = bson::to_bson(&resource).expect("Error converting struct to BSON");
    let document = bson.as_document().unwrap();

    let insert_result = database
        .collection("resources")
        .insert_one(document.to_owned(), None)
        .await
        .expect("Error inserting document into collection");

    if insert_result.inserted_id.to_string().clone().is_empty() {
        return HttpResponse::BadRequest().body("Error creating new resource.");
    }

    let mut response = CreatedResourceResponse {
        resource_id: insert_result.inserted_id.to_string(),
        group_id: group_id.clone(),
        file_put_urls: Option::None,
    };

    if let Some(f_vec) = files {
        let mut file_put_urls = Vec::<String>::new();
        for f in f_vec.iter() {
            file_put_urls.push(bucket.presign_put(format!("/{}/{}/{}", group_id, id, &f.name).as_str(), 3600, None).unwrap()); // Add file to folder for group
        }

        response = CreatedResourceResponse {
            resource_id: insert_result.inserted_id.to_string(),
            group_id,
            file_put_urls: Option::from(file_put_urls),
        };
    }

    // Add created resource to search index
    let _meili_result = index.add_documents(&[resource], Some("_id")).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string::<CreatedResourceResponse>(&response).unwrap())
}

#[get("/api/resource/get/{resource_id}")]
pub async fn fetch_resource_by_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let resource_id = ObjectId::from_str(req.match_info().get("resource_id").unwrap()).unwrap();

    let query = doc! {
        "_id": resource_id,
    };

    let result: Option<Resource> = database
        .collection("resources")
        .find_one(query, None)
        .await
        .expect("Could not fetch all documents for provided group id");

    if let Some(resource) = result {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&resource).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid resource id provided.")
}

#[get("/api/resource/get_all/{group_id}")]
pub async fn fetch_resource_by_group_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let group_id = req.match_info().get("group_id").unwrap();

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
        "group_id": group_id,
    };

    let mut cursor = database
        .collection("resources")
        .find(query, None)
        .await
        .expect("Could not fetch all documents for provided group id");

    let mut results: Vec<Resource> = Vec::new();
    while let Some(resource) = cursor.next().await {
        results.push(bson::from_document::<Resource>(resource.expect("Error")).expect("Error"))
    }

    if results.is_empty() {
        return HttpResponse::BadRequest().body("Group does not contain any resources.");
    }

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string::<Vec<Resource>>(&results).unwrap())
}

#[get("/api/resource/getByUserId/{user_id}")]
pub async fn fetch_resource_by_user_id(
    database: web::Data<Database>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.match_info().get("user_id").unwrap();

    let query = doc! {
        "user_id": user_id,
    };

    let mut cursor = database
        .collection("resources")
        .find(query, None)
        .await
        .expect("Could not fetch all documents for provided user id");

    let mut results: Vec<Resource> = Vec::new();
    while let Some(resource) = cursor.next().await {
        results.push(bson::from_document::<Resource>(resource.expect("Error")).expect("Error"))
    }

    if results.is_empty() {
        return HttpResponse::BadRequest().body("User has not created any resources.");
    }

    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string::<Vec<Resource>>(&results).unwrap())
}

#[post("/resource/update/{resource_id}")]
pub async fn update_resource(
    database: web::Data<Database>,
    index: web::Data<Index>,
    req: HttpRequest,
    resource_form: web::Json<ResourceForm>,
) -> impl Responder {

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if !check_user_in_group(authorized.user_id.clone().unwrap(), resource_form.group_id.clone(), &database).await {
        return HttpResponse::Unauthorized().body("Logged in user not in group.");
    }
    //////////////////////////////////////////////////////////////////////////

    let resource_id = req.match_info().get("resource_id").unwrap().to_string();

    let resource_form = resource_form.into_inner();
    let last_edited_at = Utc::now().naive_local();
    let query = doc! {
      "_id": resource_id.clone(),
    };

    let old_resource = database
        .collection::<Resource>("resources")
        .find_one(query.clone(), None)
        .await
        .expect("Error fetching resource from database");

    if let Some(r) = old_resource {
        if authorized.user_id.clone().unwrap().ne(&r.user_id) {
            return HttpResponse::Unauthorized().body("Unauthorized to edit resource.");
        }

        let resource = Resource::new(
            resource_id,
            authorized.user_id.unwrap(),
            r.group_id, // Keep resource in same group
            resource_form.title.clone(),
            resource_form.description.clone(),
            resource_form.subject.clone(),
            resource_form.tags.clone(),
            r.files, // Keep old resource's files
            last_edited_at,
        );

        let result = database
            .collection::<Resource>("resources")
            .replace_one(query, resource.clone(), None)
            .await
            .expect("Error updating document");

        if result.modified_count == 0 {
            return HttpResponse::BadRequest().body("No such resource exists.");
        }

        // Add updated resource to search index (will auto update as id is same)
        let _meili_result = index.add_documents(&[resource], Some("_id")).await.unwrap();

        return HttpResponse::Ok().body("Successfully updated resource.");
    }

    HttpResponse::BadRequest().body("No such resource exists.")
}

#[get("/api/resource/delete/{resource_id}")]
pub async fn delete_resource(
    database: web::Data<Database>,
    index: web::Data<Index>,
    bucket: web::Data<Bucket>,
    req: HttpRequest
) -> impl Responder {
    let resource_id = req.match_info().get("resource_id").unwrap().to_string();

    let filter = doc! {
        "_id": resource_id.clone(),
    };

    let resource = database
        .collection::<Resource>("resources")
        .find_one(filter.clone(), None)
        .await
        .expect("Error getting document");

    if resource.is_none() {
        return HttpResponse::BadRequest().body("No such resource.");
    }

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if authorized.user_id.unwrap().ne(&resource.unwrap().user_id) {
        return HttpResponse::Unauthorized().body("Logged in user not owner of resource.");
    }
    //////////////////////////////////////////////////////////////////////////

    let result = database
        .collection::<Resource>("resources")
        .delete_one(filter, None)
        .await
        .expect("Error deleting document");

    if result.deleted_count == 0 {
        return HttpResponse::BadRequest().body("No such resource exists.");
    }

    // Remove deleted resource from search index
    let _meili_result= index.delete_documents(&[resource_id.clone()]).await.unwrap();

    // Remove deleted resource's files from index
    let _bucket_result = bucket.delete_object(format!("/{}", resource_id)).await;

    HttpResponse::Ok().body("Successfully deleted resource.")
}
