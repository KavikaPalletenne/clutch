use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use actix_web::{get, post, web, Responder, HttpRequest, HttpResponse};
use crate::models::{AuthorizationCodeGrantRedirect, AccessTokenResponse, AuthorizationInformation, Group, AccessTokenRequest, NewUserRequest, UserExistsResponse};
use jsonwebtoken::EncodingKey;
use form_urlencoded::Serializer;
use std::env;
use crate::jwt::{create_auth_token, decode_auth_token};
use std::str::FromStr;
use actix_web::client::Client;
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
        .send().await.expect("Error sending GET request")
        .json::<AuthorizationInformation>().await.expect("Error parsing JSON");

    let user_id = current_user.user.id;
    let username = current_user.user.username.clone();

    let user_exists = http_client
        .get(Url::parse(format!("https://examclutch.com/api/user/protected/userExists/{}/{}", user_id.clone(), env::var("USER_SERVICE_SECRET").unwrap()).as_str()).unwrap().as_str())
        .send().await.expect("Error sending GET request")
        .json::<UserExistsResponse>().await.expect("Error parsing JSON");

    // Create new user if does not exist
    if !user_exists.exists {
        let new_user_request = NewUserRequest {
            secret: env::var("USER_SERVICE_SECRET").unwrap(),
            id: user_id.clone(),
            username: usename.clone(),
            email: "Not Used".to_string(), // Hard-coded null email as website will use Discord DMs.
        };

        let uri = Url::parse_with_params("https://examclutch.com/api/user/protected/create",
                                         &[
                                             ("secret", env::var("USER_SERVICE_SECRET").unwrap()),
                                             ("id", user_id.clone()),
                                             ("username", username.clone()),
                                             ("email", "Not Used")
                                         ])?;

        let create_user_response = http_client
            .get(uri.as_str())
            .send().await.expect("Error sending GET request");


        if !create_user_response.status().is_success() {
            return HttpResponse::BadRequest().body("Error creating user.");
        }
    }

    let token = create_auth_token(user_id, username, response, encoding_key);
    let auth_token = format!("auth_token={}; Path=/; Max-Age=604800; Secure; HttpOnly", token);

    HttpResponse::Ok()
        .header("Set-Cookie", auth_token)
        //.header("Location", "https://examclutch.com/app")
        .body(
            format!("Logged in as user {:?}", current_user.user.username.clone())
        )
}

#[get("/api/oauth2/authorize/{token}")]
pub async fn authorize(
    req: HttpRequest,
) -> impl Responder {
    let token = req.match_info().get("token").unwrap().to_string();

    let decoded_claims = decode_auth_token(token);

    if let Some(claims) = decoded_claims {
        return HttpResponse::Ok().body(claims.sub); // Return the user id
    }

    HttpResponse::Unauthorized().body("Invalid token provided.")
}

// TODO: Send a request to the user-service either through HTTP or gRPC (to learn it as well) to create a new user.
