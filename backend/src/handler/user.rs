use crate::auth::middleware::{has_user_viewing_permission, is_logged_in};
use crate::models::{NewUserForm, UpdateUserForm};
use crate::service::user;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

#[get("/api/user/{user_id}")]
pub async fn get(
    path: web::Path<String>,
    req: HttpRequest,
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

    let res = user::read(user_id, &conn).await;

    if let Ok(user) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&user).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid user id provided")
}

#[get("/api/user/username/{user_id}")]
pub async fn get_username(
    path: web::Path<String>,
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let user_id = path.into_inner();

    //     if !is_logged_in(&req, &dk) {
    //         return HttpResponse::TemporaryRedirect()
    //             .append_header(("Location", "https://examclutch.com/login"))
    //             .finish(); // Redirect to login
    //     }
    let res = user::read(user_id, &conn).await;

    if let Ok(user) = res {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("{{\"username\": \"{}\"}}", user.username).to_string());
    }
    HttpResponse::BadRequest().body("Invalid user id provided")
}

#[post("/api/user/create")]
pub async fn create_user(
    form: web::Json<NewUserForm>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    if crate::service::user::username_exists(form.username.clone(), &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Username exists");
    } else if crate::service::user::email_exists(form.username.clone(), &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Email exists");
    }

    let create_response = crate::service::user::create(form.into_inner(), &conn).await;

    if let Ok(res) = create_response {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(format!("\"username\": \"{}\"", res).to_string());
    }

    HttpResponse::BadRequest().body("Could not create new user")
}

#[post("/api/user/update/{user_id}")]
pub async fn update(
    form: web::Json<UpdateUserForm>,
    path: web::Path<String>,
    req: HttpRequest,
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

    let update_response = crate::service::user::update(user_id, form.into_inner(), &conn).await;

    if let Ok(_res) = update_response {
        return HttpResponse::Ok().body("Successfully updated user");
    }

    HttpResponse::BadRequest().body("Could not update user")
}

#[get("/api/user/delete/{user_id}")]
pub async fn delete(
    path: web::Path<String>,
    req: HttpRequest,
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

    let delete_response = crate::service::user::delete(user_id, &conn).await;

    if let Ok(_res) = delete_response {
        return HttpResponse::Ok().body("Successfully deleted user");
    }

    HttpResponse::BadRequest().body("Could not delete user or user does not exist")
}

#[get("/api/user/check_username/{username}")]
pub async fn check_username(
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    let username = path.into_inner();

    if crate::service::user::username_exists(username, &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Username exists");
    }

    HttpResponse::Ok().body("Username is available")
}

#[get("/api/user/check_email/{email}")]
pub async fn check_email(
    path: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
) -> impl Responder {
    let email = path.into_inner();

    if crate::service::user::email_exists(email, &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Email exists");
    }

    HttpResponse::Ok().body("Email is available")
}
