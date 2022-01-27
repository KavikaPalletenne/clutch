use actix_web::{HttpRequest, HttpMessage};
use actix_web::client::Client;
use crate::models::{AuthorizeResponse, GroupUser};

// returns user id if authorized or None user id if invalid
pub async fn authorize(req: &HttpRequest) -> AuthorizeResponse {
    let http_client = Client::default();
    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let token = token.value();
        let authorize_uri = format!("http://localhost:443/api/oauth2/authorize/{}", token);
        let mut response = http_client // TODO: Convert this to a local function call to the oauth2 file
            .get(authorize_uri).send()
            .await.expect("Error sending GET request");

        if response.status().is_success() {
            let response_json = response.json::<AuthorizeResponse>().await.expect("Error parsing JSON");
            return response_json; // TODO: Return response from the oauth2-service as a json for easier extracting
        }

    }

    // return
    AuthorizeResponse {
        user_id: Option::None,
        username: String::new(),
    }
}

// pub fn find_and_remove_from_vector<T>(mut vector: &Vec<T>, item: GroupUser) {
//     if let Some(index) = vector.iter().position(|value| *value == item) {
//         vector.remove(index);
//     }
// }

pub fn find_and_remove_user_from_vector(vector: &mut Vec<GroupUser>, user: GroupUser) {
    if let Some(index) = vector.iter().position(|value| *value == user) {
        vector.remove(index);
    }
}
