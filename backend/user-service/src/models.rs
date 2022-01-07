use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub secret: String, // Secret that is shared with the oauth2-service
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserExistsResponse {
    pub exists: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeResponse {
    pub user_id: Option<String>,
}
