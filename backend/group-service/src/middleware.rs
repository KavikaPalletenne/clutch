use actix_web::{HttpRequest, HttpMessage};
use actix_web::client::Client;

// returns user id if authorized or "Unauthorized" if invalid
pub async fn authorize(req: HttpRequest, http_client: Client) -> String {
    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let authorize_uri = format!("https://localhost/api/oauth2/authorize/{}", token);
        let response = http_client
            .get(authorize_uri).send()
            .await.expect("Error sending GET request");

        if response.status().is_success() {
            return response.body(); // TOOD: Return response from the oauth2-service as a json for easier extracting
        }

    }

    "Unauthorized".to_string()
}
