use actix_web::{HttpRequest, Responder, get, post, HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub author: String,
    pub group: String,
}


#[get("/cdn/file/{id}")]
pub async fn download_file(req: HttpRequest) -> impl Responder {
    // TODO: Think of a way to add authentication - maybe with a data struct that has user id or group id
    let id = req.match_info().get("id").unwrap();
    HttpResponse::PermanentRedirect()
        .header("Location", format!("https://excl.syd1.digitaloceanspaces.com/{}", id))
        .body("Unable to find file.")
}

#[post("/cdn/file/upload")]
pub async fn upload_file(
    web::Query(info): web::Query<FileUploadRequest>,
    // TODO: How to accept multipart uploads
    req: HttpRequest,
) -> impl Responder {

    // TODO: Think of a way to add authentication - maybe with a data struct that has user id or group id and make it a query
    HttpResponse::BadRequest().body("Unable to upload file.")
}
