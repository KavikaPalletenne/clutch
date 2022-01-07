use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use actix_web::{get, web, Responder, HttpRequest, HttpResponse};
use crate::models::{AuthorizationCodeGrantRedirect, AccessTokenResponse, Group, AccessTokenRequest, UserExistsResponse, DiscordUser, AuthorizeResponse, GuildResponse, PartialGuild};
use jsonwebtoken::EncodingKey;
use std::env;
use crate::jwt::{create_auth_token, decode_auth_token};
use actix_web::client::{Client};
use std::str;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // rename to _id and use and document id in database
    id: Option<ObjectId>,
    oauth2_id: String, // user id supplied from Google/Discord etc.
    username: String,  // displayed as @<username>
    email: String,
    groups: Vec<Group>, // id of group that the user is a part of
}

#[get("/api/oauth2/redirect")]
pub async fn user_registration(
    web::Query(info): web::Query<AuthorizationCodeGrantRedirect>,
    encoding_key: web::Data<EncodingKey>,
) -> impl Responder {
    let code = info.code;
    let encoding_key = encoding_key.get_ref();
    let http_client = Client::default();

    let body = AccessTokenRequest {
        client_id: env::var("CLIENT_ID").expect("Error").to_string(),
        client_secret: env::var("CLIENT_SECRET").expect("Error").to_string(),
        grant_type: "authorization_code".to_string(),
        code: code.to_string(),
        redirect_uri: "https://localhost/api/oauth2/redirect".to_string(),
    };

    //print!("Encoded Body: {:?}", encoded_body);
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

    println!("Bearer token: {}", bearer_token);

    let current_user = http_client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", bearer_token)
        .send().await.expect("Error sending GET request")
        .json::<DiscordUser>().await.expect("Error parsing JSON");

    println!("Current user: {:?}", current_user);
    let user_id = current_user.id;
    let username = current_user.username.clone();
    let email = current_user.email;

    let exists_url = Url::parse(&*format!("http://localhost:442/api/user/protected/userExists/{}/{}", user_id.clone(), env::var("USER_SERVICE_SECRET").unwrap())).unwrap().to_string();

    let user_exists = http_client
        .get(exists_url)
        .send().await.expect("Error sending GET request")
        .json::<UserExistsResponse>().await.expect("Error parsing JSON");

    // Create new user if does not exist
    if !user_exists.exists {
        let uri = Url::parse_with_params("http://localhost:442/api/user/protected/create",
                                         &[
                                             ("secret", env::var("USER_SERVICE_SECRET").unwrap()),
                                             ("id", user_id.clone()),
                                             ("username", username.clone()),
                                             ("email", email.clone())
                                         ]).expect("Error parsing URL");

        let create_user_response = http_client
            .get(uri.as_str())
            .send().await.expect("Error sending GET request");


        if !create_user_response.status().is_success() {
            return HttpResponse::BadRequest().body("Error creating user.");
        }
    }

    let token = create_auth_token(user_id, username, response, encoding_key);
    let auth_token = format!("auth_token={}; Path=/; Max-Age=604800; Secure; HttpOnly", token);

    HttpResponse::PermanentRedirect()
        .header("Set-Cookie", auth_token)
        .header("Location", "https://examclutch.com/app")
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
        let mut current_user_guilds = http_client
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
