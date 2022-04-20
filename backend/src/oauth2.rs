use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use actix_web::{get, web, Responder, HttpRequest, HttpResponse};
use crate::models::{AuthorizationCodeGrantRedirect, AccessTokenResponse, AccessTokenRequest, DiscordUser, AuthorizeResponse, PartialGuild, NewUserRequest};
use jsonwebtoken::EncodingKey;
use std::{env, time};
use crate::jwt::{create_auth_token, decode_auth_token};
use actix_web::client::{Client};
use std::str;
use cookie::{Cookie, SameSite};
use mongodb::Database;
use crate::user::{create_user_service, user_exists_service, User};

#[get("/api/oauth2/redirect")]
pub async fn user_registration(
    web::Query(info): web::Query<AuthorizationCodeGrantRedirect>,
    database: web::Data<Database>,
    encoding_key: web::Data<EncodingKey>,
) -> impl Responder {
    let code = info.code;
    let encoding_key = encoding_key.get_ref();
    // let http_client = Client::default();

    let connector = awc::Connector::new()
        // This is the timeout setting for connector. It's 1 second by default
        .timeout(time::Duration::from_secs(30))
        .finish();

    let http_client = awc::Client::builder()
        .connector(connector)
        // This is the timeout setting for requests. It's 5 seconds by default.
        .timeout(time::Duration::from_secs(50))
        .finish();

    let body = AccessTokenRequest {
        client_id: env::var("CLIENT_ID").expect("Error").to_string(),
        client_secret: env::var("CLIENT_SECRET").expect("Error").to_string(),
        grant_type: "authorization_code".to_string(),
        code: code.to_string(),
        redirect_uri: "http://127.0.0.1:443/api/oauth2/redirect".to_string(),
    };

    // return HttpResponse::Ok()
    //     .header("Content-Type", "application/json")
    //     .body(serde_json::to_string::<AccessTokenRequest>(&body).unwrap());

    // println!("Encoded Body: {:?}", body);
    let response = http_client
        .post("https://discord.com/api/oauth2/token")
        .send_form::<AccessTokenRequest>(&body)
        .await.expect("Error sending POST request").json::<AccessTokenResponse>().await.expect("Error parsing JSON");

    let bearer_token = format!("Bearer {}", response.access_token);
    // let current_user = http_client
    //     .get("https://discord.com/api/oauth2/@me")
    //     .header("Authorization", bearer_token)
    //     .send().await.expect("Error sending GET request")
    //     .json::<AuthorizationInformation>().await.expect("Error parsing JSON");

    // println!("Bearer token: {}", bearer_token);

    let current_user = http_client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", bearer_token)
        .send().await.expect("Error sending GET request")
        .json::<DiscordUser>().await.expect("Error parsing JSON");

    // println!("Current user: {:?}", current_user);
    let user_id = current_user.id.clone();
    let username = current_user.username.clone();
    let email = current_user.email;

    let user_exists = user_exists_service(&user_id, database.get_ref()).await;

    // Create new user if does not exist
    if !user_exists {
        // let uri = Url::parse_with_params("https://localhost/api/user/protected/create",
        //                                  &[
        //                                      ("secret", env::var("USER_SERVICE_SECRET").unwrap()),
        //                                      ("id", user_id.),
        //                                      ("username", username.clone()),
        //                                      ("email", email.clone())
        //                                  ]).expect("Error parsing URL");

        let user_request = NewUserRequest {
            secret: "N/A".to_string(),
            id: user_id.clone(),
            username: username.clone(),
            email
        };
        let create_user_response = create_user_service(user_request, &database).await;


        if !create_user_response {
            return HttpResponse::BadRequest().body("Error creating user.");
        }
    }

    let token = create_auth_token(user_id.clone(), username.clone(), response, encoding_key);
    // TODO: Add security features to this cookie before production deployment
    // let auth_token = format!("auth_token={}; Path=/api; Max-Age=604800; HttpOnly; Secure; SameSite=None; Domain=127.0.0.1; Port=443; Port=3000;", token);
    // let user_id_token = format!("user_id={}; Path=/; Max-Age=604800; Domain=127.0.0.1; Port=443; Port=3000", user_id);
    let auth_cookie = Cookie::build("auth_token", token)
        .domain("127.0.0.1")
        .path("/")
        .secure(false)
        .http_only(true)
        // .same_site(SameSite::Strict)
        .max_age(cookie::time::Duration::new(604800, 0))
        .finish();
    let user_id_cookie = Cookie::build("user_id", user_id)
        .domain("127.0.0.1")
        .path("/")
        .secure(false)
        .http_only(false)
        // .same_site(SameSite::None)
        .max_age(cookie::time::Duration::new(604800, 0))
        .finish();



    HttpResponse::PermanentRedirect()
        .header("Set-Cookie", auth_cookie.to_string())
        .header("Set-Cookie", user_id_cookie.to_string())
        // .header("Location", "https://examclutch.com/app")
        .header("Location", "http://localhost:3000/app")
        .body(
            format!("Logged in as user {:?}", current_user.username.clone())
        )
}

#[get("/api/oauth2/authorize/{token}")]
pub async fn authorize(
    req: HttpRequest,
) -> impl Responder {
    let token = req.match_info().get("token").unwrap().to_string();

    let decoded_claims = decode_auth_token(token);

    if let Some(claims) = decoded_claims {
        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(
            &AuthorizeResponse {
                user_id: Option::from(claims.sub),
                username: claims.username,
            }).unwrap()
        ); // Return the user id as a AuthorizeResponse JSON
    }

    HttpResponse::Unauthorized().body("Invalid token provided.")
}

#[get("/api/oauth2/guilds/{token}")]
pub async fn get_user_guilds(
    req: HttpRequest,
) -> impl Responder {
    let token = req.match_info().get("token").unwrap().to_string();
    let decoded_claims = decode_auth_token(token);

    if let Some(claims) = decoded_claims {
        let access_token = claims.access_token_response.access_token;
        let bearer_token = format!("Bearer {}", access_token);

        let http_client = Client::default();
        let current_user_guilds = http_client
            .get("https://discord.com/api/users/@me/guilds")
            .header("Authorization", bearer_token)
            .send().await.expect("Error sending GET request")
            .json::<Vec::<PartialGuild>>().await.expect("Error parsing JSON");

        return HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&current_user_guilds).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid token provided.")
}
