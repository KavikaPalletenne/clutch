use std::str::FromStr;
use actix_web::{HttpRequest, Responder, get, post, HttpResponse, web};
use mongodb::Database;
use s3::Bucket;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::file::{direct_upload, DirectUploadResponse, update_resource_files};
use crate::middleware::authorize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CdnFile {
    pub id: Uuid,
    pub size: i64, // size in bytes
    pub title: String,
    pub author: String, // user's id
    pub group: String, // group id of group file was uploaded in
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileUploadRequest {
    pub resource_id: String,
    pub file_name: String,
    pub file_size: String,
    pub file_id: String,
}


#[get("/cdn/file/{id}")]
pub async fn download_file(req: HttpRequest) -> impl Responder {
    // TODO: Think of a way to add authentication - maybe with a data struct that has user id or group id
    let id = req.match_info().get("id").unwrap();
    HttpResponse::PermanentRedirect()
        .header("Location", format!("https://excl.syd1.digitaloceanspaces.com/{}", id))
        .body("Unable to find file.")
}



#[get("/cdn/file/get_upload_url/{group_id}")]
pub async fn get_upload_url(
    bucket: web::Data<Bucket>,
    req: HttpRequest,
) -> impl Responder {

    // TODO: Check whether the user belongs to the group

    let group_id = req.match_info().get("group_id").unwrap().to_string();
    let response = direct_upload(group_id, &bucket);

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(serde_json::to_string::<DirectUploadResponse>(&response).unwrap())
}

#[get("/cdn/file/uploaded")]
pub async fn uploaded_file(
    database: web::Data<Database>,
    req: HttpRequest,
    web::Query(file_request): web::Query<FileUploadRequest>,
) -> impl Responder {

    let res = update_resource_files(
        file_request.resource_id,
        file_request.file_name,
        i64::from_str(&*file_request.file_size).unwrap(),
        file_request.file_id,
        &database
    ).await;

    HttpResponse::Ok()
        .body("Successfully updated files for resource.")
}



// #[post("/cdn/file/upload")]
// pub async fn upload_file(
//     web::Query(info): web::Query<FileUploadRequest>,
//     // TODO: How to accept multipart uploads
//     req: HttpRequest,
// ) -> impl Responder {
//
//     // TODO: Think of a way to add authentication - maybe with a data struct that has user id or group id and make it a query
//     HttpResponse::BadRequest().body("Unable to upload file.")
// }
