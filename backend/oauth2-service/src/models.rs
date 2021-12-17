use uuid::Uuid;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationCodeGrantRedirect {
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // rename to _id and use and document id in database
    id: Option<ObjectId>,
    name: String,
    members: Vec<ObjectId>,        // id's of users that are members
    administrators: Vec<ObjectId>, // id's of users that are administrators
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationJwtPayload {
    pub iss: String, // issuer
    pub sub: String, // subject (user's ObjectId from MongoDB)
    pub jti: Uuid, // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    pub username: String, // username
    pub access_token: AccessTokenResponse,
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
    pub user: DiscordUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: u64,
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
