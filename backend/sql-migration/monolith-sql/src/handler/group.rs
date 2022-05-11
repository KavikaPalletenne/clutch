use actix_web::{HttpRequest, HttpResponse, Responder, web, get, post};
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;
use crate::auth::middleware::{get_user_id, has_group_viewing_permission, is_logged_in};
use crate::models::NewGroupForm;
use crate::service::group;
use crate::service::group::user_in_group;

#[get("/api/group/{group_id}")]
pub async fn get(
    path: web::Path<String>,
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    let group_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    } else if !has_group_viewing_permission(group_id.clone(), &req, &conn, &dk).await.expect("Error") {
        return HttpResponse::Unauthorized().finish();
    }

    let res = group::read(group_id.clone(), &conn).await;

    if let Ok(group) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&group).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid group id provided")
}

#[get("/api/group/name/{group_id}")]
pub async fn get_name(
    path: web::Path<String>,
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    let group_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    }
    let res = group::read(group_id, &conn).await;

    if let Ok(group) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("\"name\": \"{}\"", group.name).to_string());
    }

    HttpResponse::BadRequest().body("Invalid group id provided")
}

#[post("/api/group/create")]
pub async fn create_group(
    req: HttpRequest,
    form: web::Json<NewGroupForm>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    let principal = get_user_id(&req, &dk);

    if let Some(creator) = principal {
        let create_response = group::create(form.into_inner(), creator, &conn).await;

        if let Ok(created_group_id) = create_response {
            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(format!("\"id\": \"{}\"", created_group_id).to_string());
        }
    }

    HttpResponse::BadRequest().body("Could not create group")
}

// TODO: update group function
// TODO: delete group function

#[post("/api/group/join/{group_id}")]
pub async fn join_group(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    }

    let group_id = path.into_inner();

    let principal = get_user_id(&req, &dk);

    if let Some(user_id) = principal {
        if user_in_group(user_id.clone(), group_id.clone(), &conn).await.expect("Error") {
            return HttpResponse::BadRequest().body("Already joined group");
        }
        let res = crate::service::group::join_group(group_id, user_id, &conn).await;
        if let Ok(_) = res {
            return HttpResponse::Ok().body("Successfully joined group");
        }
    }

    HttpResponse::BadRequest().body("Could not join group")
}

#[post("/api/group/leave/{group_id}")]
pub async fn leave_group(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>
) -> impl Responder {
    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login")).finish() // Redirect to login
    }

    let group_id = path.into_inner();

    let principal = get_user_id(&req, &dk);

    if let Some(user_id) = principal {
        if !user_in_group(user_id.clone(), group_id.clone(), &conn).await.expect("Error") {
            return HttpResponse::BadRequest().body("Not in group");
        }
        let res = crate::service::group::leave_group(group_id.clone(), user_id.clone(), &conn).await;
        if let Ok(_) = res {
            if user_in_group(user_id.clone(), group_id.clone(), &conn).await.expect("Error") {
                return HttpResponse::BadRequest().body("Could not leave group");
            }
            return HttpResponse::Ok().body("Successfully left group");
        }
    }

    HttpResponse::BadRequest().body("Could not leave group")
}
