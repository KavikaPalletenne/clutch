use actix_web::{get, web, Responder, HttpRequest, HttpResponse};
use crate::models::{AuthorizationCodeGrantRedirect, AccessTokenResponse, AccessTokenRequest, DiscordUser, PartialGuild, NewUserRequest, AuthorizeResponse};
use jsonwebtoken::EncodingKey;
use std::{env, time};
use crate::jwt::{create_auth_token, decode_auth_token};
use awc::{Client, Connector};
use cookie::{Cookie};
use crate::grpc_user::{
    create_user_service,
    user_exists_service
};

#[get("/api/auth/oauth2/discord/redirect")]
pub async fn redirect(
    web::Query(info): web::Query<AuthorizationCodeGrantRedirect>,
    encoding_key: web::Data<EncodingKey>,
) -> impl Responder {
    let code = info.code;
    let encoding_key = encoding_key.get_ref();
    // let http_client = Client::default();

    let connector = Connector::new()
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
        redirect_uri: "https://api.examclutch.com/api/auth/oauth2/redirect".to_string(),
    };

    let response = http_client
        .post("https://discord.com/api/oauth2/token")
        .send_form::<AccessTokenRequest>(&body)
        .await.expect("Error sending POST request").json::<AccessTokenResponse>().await.expect("Error parsing JSON");

    let bearer_token = format!("Bearer {}", response.access_token);


    let current_user = http_client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", bearer_token)
        .send().await.expect("Error sending GET request")
        .json::<DiscordUser>().await.expect("Error parsing JSON");

    let user_id = current_user.id.clone();
    let username = current_user.username.clone();
    let email = current_user.email;
    // TODO: Switch over to using grpc
    let user_exists = user_exists_service(&user_id).await;

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
        }; // TODO: Switch over to using grpc
        let create_user_response = create_user_service(user_request).await; // TODO: Need endpoint for this from user service


        if !create_user_response {
            return HttpResponse::BadRequest().body("Error creating user.");
        }
    }

    let token = create_auth_token(user_id.clone(), username.clone(), response, encoding_key);

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
    // let client_user_id_cookie = Cookie::build("user_id", user_id)
    //     .domain("examclutch.com")
    //     .path("/")
    //     .secure(false)
    //     .http_only(false)
    //     // .same_site(SameSite::None)
    //     .max_age(cookie::time::Duration::new(604800, 0))
    //     .finish();



    HttpResponse::PermanentRedirect()
        .append_header(("Set-Cookie", auth_cookie.to_string()))
        .append_header(("Set-Cookie", user_id_cookie.to_string()))
        // .append_header("Set-Cookie", client_user_id_cookie.to_string())
        .append_header(("Location", "https://examclutch.com/app"))
        //.append_header("Location", "http://127.0.0.1:3000/app")
        .body(
            format!("Logged in as user {:?}", current_user.username.clone())
        )
}

#[get("/api/auth/oauth2/authorize/{token}")]
pub async fn authorize(
    req: HttpRequest,
) -> impl Responder {
    let token = req.match_info().get("token").unwrap().to_string();

    let decoded_claims = decode_auth_token(token);

    if let Some(claims) = decoded_claims {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(
                &AuthorizeResponse {
                    user_id: Option::from(claims.sub),
                    username: claims.username,
                }).unwrap()
            ); // Return the user id as a AuthorizeResponse JSON
    }

    HttpResponse::Unauthorized().body("Invalid token provided.")
}

#[get("/api/auth/oauth2/discord/guilds/{token}")]
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
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&current_user_guilds).unwrap());
    }

    HttpResponse::BadRequest().body("Invalid token provided.")
}
