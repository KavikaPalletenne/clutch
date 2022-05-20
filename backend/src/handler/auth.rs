use crate::auth::jwt::{create_auth_token, decode_auth_token};
use crate::models::{LoginForm, NewUserForm};
use crate::service::user::{create, email_exists, get_by_email, username_exists};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use cookie::Cookie;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sea_orm::DatabaseConnection;
use crate::service::hashing::verify;

#[post("/api/auth/register")]
pub async fn register(
    req: HttpRequest,
    form: web::Json<NewUserForm>,
    conn: web::Data<DatabaseConnection>,
    ek: web::Data<EncodingKey>,
) -> impl Responder {
    if username_exists(form.clone().username, &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Username exists");
    } else if email_exists(form.clone().email, &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Email exists");
    }

    if let Ok(id) = create(form.clone(), &conn).await {
        return create_login_response(form.clone().username, id, ek.get_ref());
    }

    HttpResponse::BadRequest().body("Unable to register")
}

#[post("/api/auth/login")]
pub async fn login(
    req: HttpRequest,
    form: web::Json<LoginForm>,
    conn: web::Data<DatabaseConnection>,
    ek: web::Data<EncodingKey>,
) -> impl Responder {
    if !email_exists(form.clone().email, &conn)
        .await
        .expect("Error")
    {
        return HttpResponse::BadRequest().body("Invalid credentials");
    }

    if let Ok(user) = get_by_email(form.clone().email, &conn).await {
        if verify(form.clone().password, user.clone().password) {
            return create_login_response(user.username, user.id, ek.get_ref());
        }
    }

    HttpResponse::BadRequest().body("Invalid credentials")
}

#[get("/api/auth/authorize")]
pub async fn authorize(
    req: HttpRequest,
    dk: web::Data<DecodingKey>,
) -> impl Responder {
    let token = req.cookie("auth_token");

    if let Some(cookie) = token {
        if let Some(_) = decode_auth_token(cookie.to_string(), dk.get_ref()) {
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub fn create_login_response(username: String, user_id: String, ek: &EncodingKey) -> HttpResponse {
    let token = create_auth_token(user_id.clone(), username, ek);
    let auth_cookie = Cookie::build("auth_token", token)
        .domain("examclutch.com")
        .path("/")
        .secure(true)
        .http_only(true)
        // .same_site(SameSite::Strict)
        .max_age(cookie::time::Duration::new(604800, 0)) // 7 days expiry
        .finish();
    let user_id_cookie = Cookie::build("user_id", user_id.clone())
        .domain("examclutch.com")
        .path("/")
        .secure(true)
        .http_only(false)
        // .same_site(SameSite::None)
        .max_age(cookie::time::Duration::new(604800, 0)) // 7 days expiry
        .finish();

    HttpResponse::Ok()
        .append_header(("Set-Cookie", auth_cookie.to_string()))
        .append_header(("Set-Cookie", user_id_cookie.to_string()))
        .body("Successfully logged in.")
}
