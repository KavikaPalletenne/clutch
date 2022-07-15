use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// Sent for delete and edit operations regarding to user
#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthenticationJwtPayload {
    pub iss: String,      // issuer
    pub sub: i64,         // subject (user's id)
    pub jti: Uuid,        // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    // For display
    pub username: String,    // username
    pub avatar_hash: String, // url to Discord avatar
}

// Sent for resource create operation
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResourceJwtPayload {
    pub iss: String,      // issuer
    pub sub: i64,         // subject (user's id)
    pub jti: Uuid,        // id
    pub aud: Vec<String>, // audience (uri the JWT is meant for)

    // Time-based validity
    pub exp: i64, // expiry (UNIX timestamp)
    pub nbf: i64, // not-valid-before (UNIX timestamp)
    pub iat: i64, // issued-at (UNIX timestamp)

    // For AuthZ
    pub group_id: i64,

    // For display
    pub group_name: String,
    pub username: String,    // username
    pub avatar_hash: String, // url to Discord avatar
}

pub fn generate_user_token(
    user_id: i64,
    username: String,
    avatar_hash: String,
    encoding_key: &EncodingKey,
) -> String {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiry = i64::try_from((current_time + Duration::from_secs(3600)).as_secs()).unwrap(); // Expiry is 1 hour
    let claims = UserAuthenticationJwtPayload {
        iss: "examclutch".to_string(),
        sub: user_id,
        jti: Uuid::new_v4(),
        aud: vec!["*.examclutch.com".to_string()],
        exp: expiry,
        nbf: i64::try_from(current_time.as_secs()).unwrap(),
        iat: i64::try_from(current_time.as_secs()).unwrap(),
        username,
        avatar_hash,
    };

    encode(&Header::default(), &claims, encoding_key).unwrap()
}

pub fn generate_create_resource_token(
    user_id: i64,
    group_id: i64,
    group_name: String,
    username: String,
    avatar_hash: String,
    encoding_key: &EncodingKey,
) -> String {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiry = i64::try_from((current_time + Duration::from_secs(3600)).as_secs()).unwrap(); // Expiry is 1 hour
    let claims = CreateResourceJwtPayload {
        iss: "examclutch".to_string(),
        sub: user_id,
        jti: Uuid::new_v4(),
        aud: vec!["*.examclutch.com".to_string()],
        exp: expiry,
        nbf: i64::try_from(current_time.as_secs()).unwrap(),
        iat: i64::try_from(current_time.as_secs()).unwrap(),
        group_id,
        group_name,
        username,
        avatar_hash,
    };

    encode(&Header::default(), &claims, encoding_key).unwrap()
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
        Err(_err) => None::<UserAuthenticationJwtPayload>,
    };
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
        Err(_err) => None::<CreateResourceJwtPayload>,
    };
}
