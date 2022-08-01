use crate::auth::jwt::decode_create_resource_token;
use crate::auth::middleware::{
    get_user_id, has_group_viewing_permission, has_resource_viewing_permission, is_logged_in,
};
use crate::models::{
    CreatedResourceResponse, PageQuery, Resource, ResourceForm, SearchResource, TokenQuery,
};
use crate::service;
use crate::service::group;
use crate::service::resource::read;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
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
    web::Query(page_query): web::Query<PageQuery>,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let group_id = path.into_inner();
    let group_res = group::read(group_id.clone(), &conn).await;

    // Check if group exists
    if let Ok(group) = group_res {
        let res = service::resource::get_resource_by_group(
            group_id.clone(),
            page_query.num_per_page,
            page_query.page,
            &conn,
        )
        .await;

        if let Ok(resource) = res {
            if group.private {
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
            }
            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(serde_json::to_string::<Vec<Resource>>(&resource).unwrap());
        }
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

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    } else if !has_group_viewing_permission(form.group_id.clone(), &req, &conn, &dk)
        .await
        .expect("Error")
    {
        return HttpResponse::Unauthorized().finish();
    }

    let mut form = form.clone();
    form.user_id = get_user_id(&req, &dk).unwrap();

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

        let search_document = SearchResource {
            id: created_resource_id.clone().to_string(),
            user_id: form.user_id,
            group_id: form.group_id,
            title: form.title,
            description: form.description,
            subject: form.subject,
            tags: form.tags,
            files: form.files,
            last_edited_at: Utc::now(),
        };

        let meili_result = index
            .add_documents::<SearchResource>(&[search_document.clone()], Some("id"))
            .await;

        if let Ok(_) = meili_result {
            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(serde_json::to_string::<CreatedResourceResponse>(&response).unwrap());
        }

        service::resource::delete(created_resource_id.clone(), &conn)
            .await
            .unwrap();
        let _delete_result = index.delete_document(created_resource_id).await;
    }

    HttpResponse::BadRequest().body("Could not create new resource")
}

#[get("/api/resource/delete/{id}")]
pub async fn delete_resource(
    req: HttpRequest,
    path: web::Path<i64>,
    dk: web::Data<DecodingKey>,
    conn: web::Data<DatabaseConnection>,
    bucket: web::Data<Bucket>,
    index: web::Data<Index>,
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
    let user_id = get_user_id(&req, dk.get_ref());

    if let Some(uid) = user_id {
        let res = read(resource_id.clone(), &conn).await;
        if let Ok(resource) = res {
            if resource.clone().user_id.eq(&uid) {
                service::resource::delete(resource.clone().id.parse::<i64>().unwrap(), &conn)
                    .await
                    .unwrap();
                let _delete_result = index.delete_document(resource.clone().id).await;

                if let Some(files) = resource.clone().files {
                    for f in files {
                        let _obj_delete_result = bucket
                            .delete_object(format!(
                                "/{}/{}/{}",
                                resource.clone().group_id,
                                resource.clone().id,
                                f.name
                            ))
                            .await;
                    }
                }
                return HttpResponse::Ok().body("Successfully deleted resource");
            }
        }
    }

    return HttpResponse::BadRequest().body("No such resource");
}
// TODO: update endpoint
// TODO: delete endpoint

/////////////////////////
// Discord Bot Endpoints
////////////////////////

#[post("/api/discord/resource/create")]
pub async fn discord_create_resource(
    form: web::Json<ResourceForm>,
    web::Query(token_query): web::Query<TokenQuery>,
    dk: web::Data<DecodingKey>,
    bucket: web::Data<Bucket>,
    index: web::Data<Index>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    let form = form.into_inner();

    let token = token_query.token;

    let possible_jwt = decode_create_resource_token(token, &dk);

    if let Some(jwt) = possible_jwt {
        let mut form = form.clone();
        form.group_id = jwt.group_id;
        form.user_id = jwt.sub;
        // TODO: Check if the user already has an account, if not, create an account for them (no need for email as you can just message them via discord)
        // TODO: Create a way to init/create a group in database the first time

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

            let search_document = SearchResource {
                id: created_resource_id.clone().to_string(),
                user_id: form.user_id,
                group_id: form.group_id,
                title: form.title,
                description: form.description,
                subject: form.subject,
                tags: form.tags,
                files: form.files,
                last_edited_at: Utc::now(),
            };

            let meili_result = index
                .add_documents::<SearchResource>(&[search_document.clone()], Some("id"))
                .await;

            if let Ok(_) = meili_result {
                return HttpResponse::Ok()
                    .append_header(("Content-Type", "application/json"))
                    .body(serde_json::to_string::<CreatedResourceResponse>(&response).unwrap());
            }

            service::resource::delete(created_resource_id.clone(), &conn)
                .await
                .unwrap();
            let _delete_result = index.delete_document(created_resource_id).await;
        }

        return HttpResponse::BadRequest().body("Could not create new resource");
    }
    HttpResponse::Unauthorized().body("Invalid token provided")
}
