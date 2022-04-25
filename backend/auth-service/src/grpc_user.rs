use tonic::transport::Channel;

use user_service::user_service_client::UserServiceClient;
use user_service::{
    CreateUserRequest,
    UserExistsRequest,
};
use crate::models::NewUserRequest;

pub mod user_service {
    tonic::include_proto!("userservice");
}

pub async fn create_client(port: String) -> UserServiceClient<Channel> {
    UserServiceClient::connect(format!("http://[::1]:{}", port)).await.expect("Error calling connecting to user service")
}

pub async fn user_exists_service(user_id: &String) -> bool {
    let mut client = create_client(std::env::var("GRPC_USER_SERVICE_PORT").unwrap()).await;

    let request = tonic::Request::new(UserExistsRequest {
        user_id: user_id.into(),
    });

    let response = client.user_exists(request).await.expect("Error calling user_exists").into_inner();

    response.exists
}

pub async fn create_user_service(user_request: NewUserRequest) -> bool {
    let mut client = create_client(std::env::var("GRPC_USER_SERVICE_PORT").unwrap()).await;

    let request = tonic::Request::new(CreateUserRequest {
        id: user_request.id.into(),
        username: user_request.username.into(),
        email: user_request.email.into(),
    });

    let response = client.create_user(request).await.expect("Error calling create_user").into_inner();

    response.success
}

