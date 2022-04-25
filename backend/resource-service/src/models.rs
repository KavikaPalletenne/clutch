use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreatedResourceResponse {
    pub resource_id: String,
    pub group_id: String,
    pub file_put_urls: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    #[serde(rename = "_id", skip_serializing_if = "String::is_empty")]
    // rename to _id and use and document id in database
    pub id: String,
    pub user_id: String,  // owner
    pub group_id: String, // group it belongs to
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>, // Tags are optional
    pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
    pub last_edited_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceForm {
    // No need for a document Id as MongoDB generates an Id for the document when you insert it
    pub group_id: String,
    pub title: String,
    pub description: String,
    pub subject: String,
    pub tags: Option<Vec<String>>,            // Tags are optional
    pub files: Option<Vec<FileReference>>, // URL to the data (stored on server or on something like AWS S3)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReference {
    pub name: String, // This is the name the file will be stored under on the CDN
    pub title: String,
    pub size: i64, // Size in bytes
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeResponse {
    pub user_id: Option<String>,
    pub username: String,
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
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}
