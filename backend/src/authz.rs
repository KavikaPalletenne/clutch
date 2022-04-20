use actix_web::{HttpRequest, Responder, web, get, HttpResponse};
use bson::doc;
use mongodb::Database;
use crate::middleware::authorize;

// TODO: This
pub async fn check_user_group_viewing_perms(req: HttpRequest, database: web::Data<Database>) -> impl Responder {
    let user_response = authorize(&req).await;

    if let Some(id) = user_response.user_id {
        let query = doc! {
            "_id": id.clone()
        };

        //let user =
    }

    HttpResponse::BadRequest()
        .header("Content-Type", "text/plain")
        .body("Not logged in.")
}
