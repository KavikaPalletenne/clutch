use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

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

pub fn decode_auth_token(token: String, decoding_key: &DecodingKey) -> Option<AuthorizationJwtPayload> {
    let decode_token = decode::<AuthorizationJwtPayload>(
        token.as_str(),
        decoding_key,
        &Validation::new(Algorithm::HS256)
    );

    return match decode_token {
        Ok(token) => { Option::from(token.claims) },
        Err(_err) => { None::<AuthorizationJwtPayload> },
    };
}
