use actix_web::{HttpRequest, Responder, get, HttpResponse};

#[get("/cdn/file/{id}")]
pub async fn download_file(req: HttpRequest) -> impl Responder {


    HttpResponse::BadRequest().body("Unable to find file.")
}