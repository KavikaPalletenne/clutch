use crate::models::{CreatedResourceResponse, FileReference, IdResponse, NewResourceResponse, Resource, ResourceForm, Tag};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use chrono::{NaiveDateTime, Utc};
use mongodb::bson::doc;
use mongodb::Database;
use std::str::FromStr;
use nanoid::nanoid;
use s3::Bucket;
use tokio_stream::StreamExt;
use crate::file::direct_upload;
use crate::group::check_user_in_group;
use crate::middleware::authorize;

// A resource is any document or link to a website.
impl Resource {
    pub fn new(
        id: Option<String>,
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

/////////////////
// CRUD Functions
/////////////////

//TODO: Add Authorization checks (using authorization service) for all crud functions (check if the user has authorization to create/read/update/delete the resources)

#[post("/resource/create")]
pub async fn create_resource(
    req: HttpRequest,
    database: web::Data<Database>,
    bucket: web::Data<Bucket>,
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

    let id: Option<String> = Option::from(nanoid!());
    let resource = Resource::new(
        id,
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
            file_put_urls.push(bucket.presign_put(format!("/{}",&f.id).as_str(), 3600, None).unwrap());
        }

        response = CreatedResourceResponse {
            resource_id: insert_result.inserted_id.to_string(),
            group_id,
            file_put_urls: Option::from(file_put_urls),
        };
    }


    HttpResponse::Ok().body(serde_json::to_string::<CreatedResourceResponse>(&response).unwrap())
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
        .collection("resources")
        .find_one(query, None)
        .await
        .expect("Could not fetch all documents for provided group id");

    if let Some(resource) = result {
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
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
        .header("Content-Type", "application/json")
        .body(serde_json::to_string::<Vec<Resource>>(&results).unwrap())
}

#[get("/resource/getByUserId/{user_id}")]
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
        .header("Content-Type", "application/json")
        .body(serde_json::to_string::<Vec<Resource>>(&results).unwrap())
}

#[post("/resource/update/{resource_id}")]
pub async fn update_resource(
    database: web::Data<Database>,
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
        .collection::<Resource>("notes")
        .find_one(query.clone(), None)
        .await
        .expect("Error fetching resource from database");

    if let Some(r) = old_resource {
        if authorized.user_id.clone().unwrap().ne(&r.user_id) {
            return HttpResponse::Unauthorized().body("Unauthorized to edit resource.");
        }

        let id = Option::from(resource_id);
        let resource = Resource::new(
            id,
            authorized.user_id.unwrap(),
            resource_form.group_id,
            resource_form.title.clone(),
            resource_form.description.clone(),
            resource_form.subject.clone(),
            resource_form.tags.clone(),
            resource_form.files.clone(),
            last_edited_at,
        );

        let result = database
            .collection::<Resource>("notes")
            .replace_one(query, resource, None)
            .await
            .expect("Error updating document");

        if result.modified_count == 0 {
            return HttpResponse::BadRequest().body("No such resource exists.");
        }

        return HttpResponse::Ok().body("Successfully updated resource.");
    }

    HttpResponse::BadRequest().body("No such resource exists.")
}

#[get("/resource/delete/{resource_id}")]
pub async fn delete_resource(database: web::Data<Database>, req: HttpRequest) -> impl Responder {
    let resource_id = ObjectId::from_str(req.match_info().get("resource_id").unwrap()).unwrap();

    let filter = doc! {
        "_id": resource_id,
    };

    let resource = database
        .collection::<Resource>("notes")
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
        .collection::<Resource>("notes")
        .delete_one(filter, None)
        .await
        .expect("Error deleting document");

    if result.deleted_count == 0 {
        return HttpResponse::BadRequest().body("No such resource exists.");
    }

    HttpResponse::Ok().body("Successfully deleted resource.")
}
