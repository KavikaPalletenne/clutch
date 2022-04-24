use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationCodeGrantRedirect {
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationJwtPayload {
    pub iss: String, // issuer
    pub sub: String, // subject (user's id)
    pub jti: Uuid, // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    pub username: String, // username
    pub access_token_response: AccessTokenResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationInformation {
    pub application: DiscordApplication,
    pub scopes: Vec<String>,
    pub expires: String,
    pub user: DiscordAuthInfoUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordAuthInfoUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub banner_color: Option<i64>,
    pub accent_color: Option<i64>,
    pub locale: Option<String>,
    pub verified: bool,
    pub email: String,
    pub flags: Option<i64>,
    pub premium_type: Option<i64>,
    pub public_flags: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordApplication {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub summary: String,
    pub hook: bool,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub verify_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    pub id: String,
    pub name: String,
    pub icon: Option::<String>,
    pub owner: bool,
    pub permissions: i64,
    pub features: Vec::<String>,
    pub permissions_new: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub secret: String, // Secret that is shared with the oauth2-service
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeResponse {
    pub user_id: Option<String>,
    pub username: String,
}
