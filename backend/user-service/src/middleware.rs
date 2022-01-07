use actix_web::{HttpRequest, HttpMessage};
use actix_web::client::Client;
use crate::models::AuthorizeResponse;

// returns user id if authorized or None user id if invalid
pub async fn authorize(req: HttpRequest) -> AuthorizeResponse {
    let http_client = Client::default();
    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let authorize_uri = format!("https://localhost/api/oauth2/authorize/{}", token);
        let mut response = http_client
            .get(authorize_uri).send()
            .await.expect("Error sending GET request");

        if response.status().is_success() {
            let response_json = response.json::<AuthorizeResponse>().await.expect("Error parsing JSON");
            return response_json; // TOOD: Return response from the oauth2-service as a json for easier extracting
        }

    }

    // return
    AuthorizeResponse {
        user_id: Option::None,
    }
}
