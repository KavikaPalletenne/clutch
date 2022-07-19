use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationJwtPayload {
    pub iss: String,      // issuer
    pub sub: String,      // subject (user's id)
    pub jti: Uuid,        // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    pub username: String, // username
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

// Sent for delete and edit operations regarding to user
#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthenticationJwtPayload {
    pub iss: String,      // issuer
    pub sub: String,      // subject (user's id)
    pub jti: Uuid,        // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    // For display
    pub username: String, // username
    pub avatar_hash: String, // url to Discord avatar
}

// Sent for resource create operation
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResourceJwtPayload {
    pub iss: String,      // issuer
    pub sub: String,      // subject (user's id)
    pub jti: Uuid,        // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    // For AuthZ
    pub group_id: String,

    // For display
    pub group_name: String,
    pub username: String, // username
    pub avatar_hash: String, // url to Discord avatar
}

pub fn create_auth_token(user_id: String, username: String, encoding_key: &EncodingKey) -> String {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiry = i64::try_from((current_time + Duration::from_secs(604800)).as_secs()).unwrap(); // Expiry is 7 days (same as Discord)
    let claims = AuthorizationJwtPayload {
        iss: "examclutch".to_string(),
        sub: user_id,
        jti: Uuid::new_v4(),
        aud: vec!["*.examclutch.com".to_string()],
        exp: expiry,
        nbf: i64::try_from(current_time.as_secs()).unwrap(),
        iat: i64::try_from(current_time.as_secs()).unwrap(),
        username,
    };

    encode(&Header::default(), &claims, encoding_key).unwrap()
}

pub fn decode_auth_token(
    token: String,
    decoding_key: &DecodingKey,
) -> Option<AuthorizationJwtPayload> {
    let decode_token = decode::<AuthorizationJwtPayload>(
        token.as_str(),
        decoding_key,
        &Validation::new(Algorithm::HS256),
    );

    return match decode_token {
        Ok(token) => Option::from(token.claims),
        Err(_err) => None::<AuthorizationJwtPayload>,
    };
}

pub fn decode_user_token(
    token: String,
    decoding_key: &DecodingKey,
) -> Option<UserAuthenticationJwtPayload> {
    let decode_token = decode::<UserAuthenticationJwtPayload>(
        token.as_str(),
        decoding_key,
        &Validation::new(Algorithm::HS256),
    );

    return match decode_token {
        Ok(token) => Option::from(token.claims),
        Err(_err) => None::<UserAuthenticationJwtPayload>
    }
}

pub fn decode_create_resource_token(
    token: String,
    decoding_key: &DecodingKey,
) -> Option<CreateResourceJwtPayload> {
    let decode_token = decode::<CreateResourceJwtPayload>(
        token.as_str(),
        decoding_key,
        &Validation::new(Algorithm::HS256),
    );

    return match decode_token {
        Ok(token) => Option::from(token.claims),
        Err(_err) => None::<CreateResourceJwtPayload>
    }
}
