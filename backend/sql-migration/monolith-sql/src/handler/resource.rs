use actix_web::{HttpRequest, web, get, post, Responder, HttpResponse};
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;
use crate::auth::middleware::{get_user_id, has_group_viewing_permission, has_resource_viewing_permission, is_logged_in};
use crate::models::ResourceForm;
use crate::service;
use crate::service::resource::create;

#[get("/api/resource/{resource_id}")]
pub async fn get(
    req: HttpRequest,
    path: web::Path<i64>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    let resource_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    } else if !has_resource_viewing_permission(resource_id.clone(), &req, &conn, &dk).await.expect("Error") {
        return HttpResponse::Unauthorized().finish();
    }

    let res = service::resource::read(resource_id.clone(), &conn).await;

    if let Ok(resource) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&resource).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid resource id provided")
}

#[post("/api/resource/create")]
pub async fn create_resource(
    form: web::Json<ResourceForm>,
    req: HttpRequest,
    dk: web::Data<DecodingKey>,
    conn: web::Data<DatabaseConnection>
) -> impl Responder {
    let form = form.into_inner();
    let files = form.files.clone(); // TODO: Return urls for file upload using these

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    } else if !has_group_viewing_permission(form.group_id.clone(), &req, &conn, &dk).await.expect("Error") {
        return HttpResponse::Unauthorized().finish();
    } else if form.user_id.ne(&get_user_id(&req, &dk).unwrap()) {
        return HttpResponse::BadRequest().body("Not logged in user");
    }

    let create_response = service::resource::create(form, &conn).await;

    if let Ok(res) = create_response {

        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("\"id\": \"{}\"", res).to_string());
    }

    HttpResponse::BadRequest().body("Could not create new resource")
}

// TODO: update endpoint
// TODO: delete endpoint
