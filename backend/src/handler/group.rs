use crate::auth::middleware::{
    get_user_id, has_group_viewing_permission, has_user_viewing_permission, is_logged_in,
};
use crate::models::{CreateInviteCodeQuery, NewGroupForm};
use crate::service;
use crate::service::group;
use crate::service::group::{generate_invite_code, get_invite_code_group, read, user_in_group};
use crate::service::role::Role;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

#[get("/api/group/{group_id}")]
pub async fn get(
    path: web::Path<String>,
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let group_id = path.into_inner();
    let res = group::read(group_id.clone(), &conn).await;

    if let Ok(group) = res {
        if group.private {
            if !is_logged_in(&req, &dk) {
                return HttpResponse::Unauthorized().finish();
            } else if !has_group_viewing_permission(group_id.clone(), &req, &conn, &dk)
                .await
                .expect("Error")
            {
                return HttpResponse::Unauthorized().finish();
            }
        }
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&group).unwrap());
    }

    HttpResponse::Unauthorized().finish()
}

#[get("/api/group/name/{group_id}")]
pub async fn get_name(
    path: web::Path<String>,
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let group_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    }
    let res = group::read(group_id, &conn).await;

    if let Ok(group) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("{{\"name\": \"{}\"}}", group.name).to_string());
    }

    HttpResponse::BadRequest().body("Invalid group id provided")
}

#[post("/api/group/create")]
pub async fn create_group(
    req: HttpRequest,
    form: web::Json<NewGroupForm>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let principal = get_user_id(&req, &dk);

    if let Some(creator) = principal {
        let create_response = group::create(form.into_inner(), creator, &conn).await;

        if let Ok(created_group_id) = create_response {
            return HttpResponse::Ok()
                .append_header(("Content-Type", "application/json"))
                .body(format!("{{\"id\": \"{}\"}}", created_group_id).to_string());
        }
    }

    HttpResponse::BadRequest().body("Could not create group")
}

// TODO: update group function
// TODO: delete group function

#[post("/api/group/{group_id}/create_invite")]
pub async fn create_invite_code(
    req: HttpRequest,
    path: web::Path<String>,
    web::Query(create_code_query): web::Query<CreateInviteCodeQuery>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    }
    let creator = get_user_id(&req, &dk).unwrap();

    let group_id = path.into_inner();

    let user_permissions = Role::get_user_permissions(group_id.clone(), creator.clone(), &conn)
        .await
        .expect("Error getting user permissions");
    if user_permissions.contains(&"administrator".to_string())
        || user_permissions.contains(&"owner".to_string())
        || user_permissions.contains(&"invite_create".to_string())
    {
        let code = generate_invite_code(
            group_id.clone(),
            creator.clone(),
            create_code_query.expiry,
            &conn,
        )
        .await
        .expect("Error generating invite code");

        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("{{\"invite_code\": \"{}\"}}", code).to_string());
    }

    HttpResponse::Unauthorized().body("Not allowed to create invites in group")
}

#[post("/api/group/join/{invite_code}")]
pub async fn join_group(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    }

    let invite_code = path.into_inner();
    let possible_id = get_invite_code_group(invite_code.clone(), &conn).await;

    if let Ok(group_id) = possible_id {
        let principal = get_user_id(&req, &dk);

        if let Some(user_id) = principal {
            let group_res = read(group_id.clone(), &conn).await;

            if let Ok(_group) = group_res {
                if user_in_group(user_id.clone(), group_id.clone(), &conn)
                    .await
                    .expect("Error")
                {
                    return HttpResponse::BadRequest().body("Already joined group");
                }
                let res = crate::service::group::join_group(invite_code, user_id, &conn).await;
                if let Ok(_) = res {
                    return HttpResponse::Ok().body("Successfully joined group");
                }
            }

            return HttpResponse::BadRequest().body("No such group");
        }
    }

    HttpResponse::BadRequest().body("No such invite code")
}

#[post("/api/group/leave/{group_id}")]
pub async fn leave_group(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    }

    let group_id = path.into_inner();

    let principal = get_user_id(&req, &dk);

    if let Some(user_id) = principal {
        if !user_in_group(user_id.clone(), group_id.clone(), &conn)
            .await
            .expect("Error")
        {
            return HttpResponse::BadRequest().body("Not in group");
        }
        let res =
            crate::service::group::leave_group(group_id.clone(), user_id.clone(), &conn).await;
        if let Ok(_) = res {
            if user_in_group(user_id.clone(), group_id.clone(), &conn)
                .await
                .expect("Error")
            {
                return HttpResponse::BadRequest().body("Could not leave group");
            }
            return HttpResponse::Ok().body("Successfully left group");
        }
    }

    HttpResponse::BadRequest().body("Could not leave group")
}

#[get("/api/group/user_groups/{user_id}")]
pub async fn get_user_groups(
    req: HttpRequest,
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let user_id = path.into_inner();

    if !is_logged_in(&req, &dk) {
        return HttpResponse::TemporaryRedirect()
            .append_header(("Location", "https://examclutch.com/login"))
            .finish(); // Redirect to login
    } else if !has_user_viewing_permission(user_id.clone(), &req, &dk) {
        return HttpResponse::Unauthorized().finish();
    }

    let res = service::group::get_user_groups(user_id.clone(), &conn).await;

    if let Ok(groups) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&groups).unwrap());
    }

    HttpResponse::BadRequest().body("Could not find user")
}
