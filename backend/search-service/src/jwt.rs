use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};
use crate::models::AuthorizationJwtPayload;

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
