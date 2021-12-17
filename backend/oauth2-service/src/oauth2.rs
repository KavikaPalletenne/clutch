use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use actix_web::{get, post, web, Responder, HttpRequest, HttpResponse};
use crate::models::{AuthorizationCodeGrantRedirect, AccessTokenResponse, AuthorizationInformation, Group, AccessTokenRequest};
use jsonwebtoken::EncodingKey;
use form_urlencoded::Serializer;
use std::env;
use crate::jwt::create_auth_token;
use std::str::FromStr;
use actix_web::client::Client;
use std::str;

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

#[get("/api/oauth2/redirect")] // TOOD: Parse URL parameters
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
    let current_user = http_client
        .get("https://discord.com/api/oauth2/@me")
        .header("Authorization", bearer_token)
        .send().await.expect("Error sending GET request").json::<AuthorizationInformation>().await.expect("Error parsing json");

    let user_id = current_user.user.id;
    let username = current_user.user.username.clone();

    let token = create_auth_token(user_id, username, response, encoding_key);
    let auth_token = format!("auth_token={}; Path=/; Max-Age=604800; Secure; HttpOnly", token);

    HttpResponse::Ok()
        .header("Set-Cookie", auth_token)
        //.header("Location", "https://examclutch.com/app")
        .body(
            format!("Logged in as user {:?}", current_user.user.username.clone())
        )
}
