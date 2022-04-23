use actix_web::{HttpRequest};
use crate::models::{AuthorizeResponse};
use crate::oauth2::authorize_local;

// returns user id if authorized or None user id if invalid
pub async fn authorize(req: &HttpRequest) -> AuthorizeResponse {

    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let token = token.value().to_string();

        let response = authorize_local(token);

        if response.user_id.is_some() {
            return response;
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

pub fn find_and_remove_string_from_vector(vector: &mut Vec<String>, string: String) {
    if let Some(index) = vector.iter().position(|value| *value == string) {
        vector.remove(index);
    }
}
