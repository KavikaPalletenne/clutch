use anyhow::Result;
use tonic::{Request, Response, Status};

use auth_service::auth_service_server::AuthService;
use auth_service::{AuthorizeRequest, AuthorizeResponse};
use crate::jwt::decode_auth_token;

pub mod auth_service {
    tonic::include_proto!("authservice");
}

#[derive(Debug, Default)]
pub struct AuthServiceHandler {}

#[tonic::async_trait]
impl AuthService for AuthServiceHandler {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeResponse>, Status> {

        let decoded_claims = decode_auth_token(request.into_inner().token);

        if let Some(claims) = decoded_claims {
            let response = AuthorizeResponse {
                user_id: Option::from(claims.sub).into(),
                username: claims.username.into(),
            };

            return Ok(Response::new(response));
        }

        let response = AuthorizeResponse {
            user_id: Option::None.into(),
            username: String::new().into(),
        };

        Ok(Response::new(response))
    }
}
