// use std::fmt::Error;
// use anyhow::Result;
// use std::future::{Ready, ready};
// use actix_web::{FromRequest, HttpRequest, web};
// use actix_web::dev::Payload;
// use anyhow::bail;
// use jsonwebtoken::DecodingKey;
// use crate::auth::jwt::decode_auth_token;
// use super::super::errors::MyAuthError;
//
// pub struct Principal {
//     pub user_id: String,
//     pub groups: Vec<String>, // Authorized groups
// }
//
// impl Principal {
//     /// Returns the user id from Jwt
//     pub fn extract_user_id(req: &HttpRequest, decoding_key: &DecodingKey) -> Result<String> {
//
//         let auth_token = req.cookie("auth_token");
//
//         if let Some(token) = auth_token {
//             let token = token.value().to_string();
//
//             let possible_claims = decode_auth_token(token, decoding_key);
//
//             if let Some(claims) = possible_claims {
//                 Ok(claims.sub)
//             }
//
//         }
//
//         return bail!(MyAuthError::NoJwt);
//     }
// }
// // TODO: Proper middleware for authz - https://www.lpalmieri.com/posts/session-based-authentication-in-rust/#3-5-3-a-typed-interface-to-session
// impl FromRequest for Principal {
//     type Error = Error;
//     type Future = Ready<Result<Principal>>;
//
//     fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
//         let decoding_key = req.app_data::<web::Data<DecodingKey>>().unwrap();
//         let user_id = Principal::extract_user_id(req, decoding_key.get_ref());
//
//         if let Ok(id) = user_id {
//             return ready(Ok(Principal {
//                 user_id: id,
//                 groups: vec!["".to_string()],
//             }));
//         }
//
//         ready(bail!(MyAuthError::NoJwt))
//     }
// }
