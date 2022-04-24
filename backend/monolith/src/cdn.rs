use actix_web::{HttpRequest, Responder, get, HttpResponse, web};
use mongodb::Database;
use s3::Bucket;
// use crate::file::{direct_upload, DirectUploadResponse};
use crate::group::check_user_in_group;
use crate::middleware::authorize;


#[get("/cdn/file/{group_id}/{resource_id}/{id}")]
pub async fn download_file(req: HttpRequest, bucket: web::Data<Bucket>, database: web::Data<Database>) -> impl Responder {

    let group_id = req.match_info().get("group_id").unwrap().to_string();
    let resource_id = req.match_info().get("resource_id").unwrap().to_string();
    let id = req.match_info().get("id").unwrap();

    //////////////////////////////////////////////////////////////////////////
    // Auth //
    let authorized = authorize(&req).await;

    if authorized.user_id.is_none() {
        return HttpResponse::Unauthorized().body("Not logged in.");
    }

    if !check_user_in_group(authorized.user_id.clone().unwrap(), group_id.clone(), &database).await {
        return HttpResponse::Unauthorized().body("Logged in user not in group.");
    }
    //////////////////////////////////////////////////////////////////////////

    let file_download_url = bucket.presign_get(format!("/{}/{}/{}", group_id, resource_id, id), 3600, None).unwrap(); // 1 hour expiry
    HttpResponse::PermanentRedirect()
        .append_header(("Location", file_download_url))
        .body("Unable to find file.")
}



// #[get("/cdn/file/get_upload_url/{group_id}")]
// pub async fn get_upload_url(
//     bucket: web::Data<Bucket>,
//     database: web::Data<Database>,
//     req: HttpRequest,
// ) -> impl Responder {
//
//     let group_id = req.match_info().get("group_id").unwrap().to_string();
//
//     //////////////////////////////////////////////////////////////////////////
//     // Auth //
//     let authorized = authorize(&req).await;
//
//     if authorized.user_id.is_none() {
//         return HttpResponse::Unauthorized().body("Not logged in.");
//     }
//
//     if !check_user_in_group(authorized.user_id.clone().unwrap(), group_id.clone(), &database).await {
//         return HttpResponse::Unauthorized().body("Logged in user not in group.");
//     }
//     //////////////////////////////////////////////////////////////////////////
//
//     let response = direct_upload(group_id, &bucket);
//
//     HttpResponse::Ok()
//         .append_header(("Content-Type", "application/json"))
//         .body(serde_json::to_string::<DirectUploadResponse>(&response).unwrap())
// }
