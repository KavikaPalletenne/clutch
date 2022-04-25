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
