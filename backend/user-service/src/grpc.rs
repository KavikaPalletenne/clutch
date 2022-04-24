use anyhow::Result;
use bson::doc;
use mongodb::Database;
use tonic::{Request, Response, Status};

use user_service::user_service_server::UserService;
use user_service::{
    CreateUserRequest,
    CreateUserResponse,
    UserExistsRequest,
    UserExistsResponse,
};
use crate::user::User;

pub mod user_service {
    tonic::include_proto!("userservice");
}

#[derive(Debug)]
pub struct UserServiceHandler {
    database: Database,
}

impl UserServiceHandler {
    pub fn new(database: Database) -> Self {
        Self {
            database
        }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceHandler {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let user_request = request.into_inner();

        let user = User::new(user_request.id, user_request.username, user_request.email);

        let bson = bson::to_bson(&user).expect("Error converting struct to BSON");
        let document = bson.as_document().unwrap();

        let insert_result = self.database
            .collection("users")
            .insert_one(document.to_owned(), None)
            .await
            .expect("Error inserting document into collection");

        if insert_result.inserted_id.to_string().is_empty() {
            let response = user_service::CreateUserResponse {
                success: false.into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
            };

            return Ok(Response::new(response));
        }

        let response = user_service::CreateUserResponse {
            success: true.into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(response))
    }

    async fn user_exists(
        &self,
        request: Request<UserExistsRequest>,
    ) -> Result<Response<UserExistsResponse>, Status> {

        let user_id = request.into_inner().user_id;

        let query = doc! {
        "_id": user_id,
        };

        let result: Option<User> = self.database
            .collection("users")
            .find_one(query, None)
            .await
            .expect("Could not fetch user with provided id");

        if let Some(_) = result {
            let response = UserExistsResponse {
                exists: true.into()
            };

            return Ok(Response::new(response))
        }

        let response = UserExistsResponse {
            exists: false.into()
        };

        return Ok(Response::new(response))
    }
}
