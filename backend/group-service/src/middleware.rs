use actix_web::{HttpRequest};
use jsonwebtoken::DecodingKey;
use crate::jwt::decode_auth_token;
use crate::models::{AuthorizeResponse};

// returns user id if authorized or None user id if invalid
pub async fn authorize(req: &HttpRequest, decoding_key: &DecodingKey) -> AuthorizeResponse {

    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let token = token.value().to_string();

        let possible_claims = decode_auth_token(token, decoding_key);

        if let Some(claims) = possible_claims {
            return AuthorizeResponse {
                user_id: Option::from(claims.sub),
                username: claims.username,
            };
        }

    }

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
