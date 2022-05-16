use crate::auth::middleware::{
    get_user_id, has_group_viewing_permission, has_resource_viewing_permission, is_logged_in,
};
use crate::models::{CreatedResourceResponse, ResourceForm};
use crate::service;
use crate::service::resource::create;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::DecodingKey;
use meilisearch_sdk::indexes::Index;
use s3::Bucket;
use sea_orm::DatabaseConnection;

#[get("/api/resource/{resource_id}")]
pub async fn get(
    req: HttpRequest,
    path: web::Path<i64>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let resource_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    } else if !has_resource_viewing_permission(resource_id.clone(), &req, &conn, &dk)
        .await
        .expect("Error")
    {
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

#[get("/api/resource/get_all/{group_id}")]
pub async fn get_by_group(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let group_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    } else if !has_group_viewing_permission(group_id.clone(), &req, &conn, &dk)
        .await
        .expect("Error")
    {
        return HttpResponse::Unauthorized().finish();
    }

    let res = service::resource::get_resource_by_group(group_id.clone(), &conn).await;

    if let Ok(resource) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&resource).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid group id provided")
}

#[post("/api/resource/create")]
pub async fn create_resource(
    form: web::Json<ResourceForm>,
    req: HttpRequest,
    dk: web::Data<DecodingKey>,
    bucket: web::Data<Bucket>,
    index: web::Data<Index>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    let form = form.into_inner();
    let files = form.files.clone();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    } else if !has_group_viewing_permission(form.group_id.clone(), &req, &conn, &dk)
        .await
        .expect("Error")
    {
        return HttpResponse::Unauthorized().finish();
    } else if form.user_id.ne(&get_user_id(&req, &dk).unwrap()) {
        return HttpResponse::BadRequest().body("Not logged in user");
    }

    let create_response = service::resource::create(form.clone(), &conn).await;

    if let Ok(created_resource_id) = create_response {
        let mut file_put_urls = Vec::<String>::new();
        if let Some(f_vec) = form.clone().files {
            for f in f_vec {
                file_put_urls.push(
                    bucket
                        .presign_put(
                            format!(
                                "/{}/{}/{}",
                                form.group_id.clone(),
                                created_resource_id.clone(),
                                &f.name
                            )
                            .as_str(),
                            3600,
                            None,
                        )
                        .unwrap(),
                );
            }
        }

        let response = CreatedResourceResponse {
            resource_id: created_resource_id.clone(),
            group_id: form.clone().group_id,
            file_put_urls: Option::from(file_put_urls),
        };

        let meili_result = index
            .add_documents(&[form.clone()], Some(&created_resource_id.to_string()))
            .await;

        if let Ok(_) = meili_result {
            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(serde_json::to_string::<CreatedResourceResponse>(&response).unwrap());
        }

        service::resource::delete(created_resource_id.clone(), &conn)
            .await
            .unwrap();
    }

    HttpResponse::BadRequest().body("Could not create new resource")
}

// TODO: update endpoint
// TODO: delete endpoint
