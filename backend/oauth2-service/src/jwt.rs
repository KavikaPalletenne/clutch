use std::convert::TryFrom;
use std::env;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::models::{AuthorizationJwtPayload, AccessTokenResponse};
use bson::oid::ObjectId;


pub fn create_auth_token(user_id: String, username: String, access_token: AccessTokenResponse, encoding_key: &EncodingKey) -> String {

    // TODO: Choose a JWT signing algorithm and look into signing and encrypting (nested JWTs). Make a custom header with this.

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiry = i64::try_from((current_time + Duration::from_secs(access_token.expires_in)).as_secs()).unwrap(); // Expiry according to Discord response
    let claims = AuthorizationJwtPayload {
        iss: "examclutch".to_string(),
        sub: user_id,
        jti: Uuid::new_v4(),
        aud: vec!["*.examclutch.com".to_string()],
        exp: expiry,
        nbf: i64::try_from(current_time.as_secs()).unwrap(),
        iat: i64::try_from(current_time.as_secs()).unwrap(),
        username,
        access_token,
    };

    encode(&Header::default(), &claims, encoding_key).unwrap()
}

pub fn decode_auth_token(token: String) -> Option<AuthorizationJwtPayload> {
    let decode_token = decode::<AuthorizationJwtPayload>(
        token.as_str(),
        &DecodingKey::from_secret(env::var("JWT_SECRET").expect("Error getting JWT_SECRET").as_ref()),
        &Validation::new(Algorithm::HS256)
    );

    return match decode_token {
        Ok(token) => { Option::from(token.claims) },
        Err(_err) => { None::<AuthorizationJwtPayload> },
    };
}
