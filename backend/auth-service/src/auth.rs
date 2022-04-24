use crate::jwt::decode_auth_token;
use crate::models::AuthorizeResponse;

pub fn authorize_local(
    token: String,
) -> AuthorizeResponse {
    let decoded_claims = decode_auth_token(token);

    if let Some(claims) = decoded_claims {
        return AuthorizeResponse {
            user_id: Option::from(claims.sub),
            username: claims.username,
        };
    }

    AuthorizeResponse {
        user_id: Option::None,
        username: String::new(),
    }
}
