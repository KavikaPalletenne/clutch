use crate::auth::jwt::create_auth_token;
use crate::models::NewUserForm;
use crate::service::user::{create, email_exists, username_exists};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use cookie::Cookie;
use jsonwebtoken::EncodingKey;
use sea_orm::DatabaseConnection;

#[post("/api/auth/register")]
pub async fn register(
    req: HttpRequest,
    form: web::Json<NewUserForm>,
    conn: web::Data<DatabaseConnection>,
    ek: web::Data<EncodingKey>,
) -> impl Responder {
    if username_exists(form.clone().username, &conn).await.expect("Error") {
        return HttpResponse::BadRequest().body("Username exists");
    } else if email_exists(form.clone().email, &conn).await.expect("Error") {
        return HttpResponse::BadRequest().body("Email exists");
    }

    if let Ok(id) = create(form.clone(), &conn).await {
        return create_login_response(form.clone().username, id, ek.get_ref());
    }

    HttpResponse::BadRequest().body("Unable to register")
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

    HttpResponse::TemporaryRedirect()
        .append_header(("Set-Cookie", auth_cookie.to_string()))
        .append_header(("Set-Cookie", user_id_cookie.to_string()))
        .append_header(("Location", "https://examclutch.com/app"))
        .body("Successfully logged in.")
}
