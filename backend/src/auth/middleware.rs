use crate::auth::jwt::decode_auth_token;
use actix_web::{web, HttpRequest};
use anyhow::Result;
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

// TODO: Proper middleware for authz - https://www.lpalmieri.com/posts/session-based-authentication-in-rust/#3-5-3-a-typed-interface-to-session
// pub async fn get_principal(
//     mut req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
//     let pool = req.app_data::<web::Data<DatabaseConnection>>().unwrap();
//
//     Ok(())
// }

pub fn is_logged_in(req: &HttpRequest, decoding_key: &DecodingKey) -> bool {
    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let token = token.value().to_string();

        let possible_claims = decode_auth_token(token, decoding_key);

        if let Some(_claims) = possible_claims {
            return true;
        }
    }

    false
}

// // returns user id if authorized or None user id if invalid
// pub fn authorize(req: &HttpRequest, decoding_key: &DecodingKey) -> AuthorizeResponse {
//
//     let auth_token = req.cookie("auth_token");
//
//     if let Some(token) = auth_token {
//         let token = token.value().to_string();
//
//         let possible_claims = decode_auth_token(token, decoding_key);
//
//         if let Some(claims) = possible_claims {
//             return AuthorizeResponse {
//                 user_id: Option::from(claims.sub),
//                 username: claims.username,
//             };
//         }
//
//     }
//
//     AuthorizeResponse {
//         user_id: Option::None,
//         username: String::new(),
//     }
// }

pub fn get_user_id(req: &HttpRequest, decoding_key: &DecodingKey) -> Option<String> {
    let auth_token = req.cookie("auth_token");

    if let Some(token) = auth_token {
        let token = token.value().to_string();

        let possible_claims = decode_auth_token(token, decoding_key);

        if let Some(claims) = possible_claims {
            return Option::from(claims.sub);
        }
    }

    None
}

pub fn has_user_viewing_permission(
    user_id: String,
    req: &HttpRequest,
    decoding_key: &DecodingKey,
) -> bool {
    let token = req.cookie("auth_token");

    if let Some(t) = token {
        let t = t.value().to_string();
        let possible_claims = decode_auth_token(t, decoding_key);

        if let Some(claims) = possible_claims {
            if claims.sub.eq(&user_id) {
                return true;
            }
        }
    }
    false
}

pub async fn has_resource_viewing_permission(
    resource_id: i64,
    req: &HttpRequest,
    conn: &web::Data<DatabaseConnection>,
    decoding_key: &DecodingKey,
) -> Result<bool> {
    let token = req.cookie("auth_token");

    if let Some(t) = token {
        let t = t.value().to_string();
        let possible_claims = decode_auth_token(t, decoding_key);

        if let Some(claims) = possible_claims {
            if crate::service::resource::user_can_view_resource(claims.sub, resource_id, conn)
                .await?
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub async fn has_group_viewing_permission(
    group_id: String,
    req: &HttpRequest,
    conn: &web::Data<DatabaseConnection>,
    decoding_key: &DecodingKey,
) -> Result<bool> {
    let token = req.cookie("auth_token");

    if let Some(t) = token {
        let t = t.value().to_string();
        let possible_claims = decode_auth_token(t, decoding_key);

        if let Some(claims) = possible_claims {
            if crate::service::group::user_in_group(claims.sub, group_id, conn).await? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
