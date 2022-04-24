use anyhow::Result;
use actix_web::{HttpServer, App};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use jsonwebtoken::EncodingKey;
use tonic::transport::Server;
use crate::grpc::{
    AuthServiceHandler,
    auth_service::auth_service_server::AuthServiceServer,
};

mod discord;
mod models;
mod jwt;
mod grpc;
// mod auth;
mod grpc_user;

#[tokio::main]
async fn main() -> Result<()> {
    let actix_port = std::env::var("ACTIX_PORT").expect("Error getting ACTIX_PORT").to_string();
    let grpc_port = std::env::var("GRPC_PORT").expect("Error getting GRPC_PORT").to_string();

    // Initialise logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Initialised logger");

    // Initialise JWT settings
    let jwt_secret = std::env::var("JWT_SECRET").expect("Error getting JWT_SECRET").to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    println!("Initialised JWT settings");

    // Initialise gRPC server
    println!("Starting grpc server on port {}", grpc_port.clone());
    let addr = format!("[::1]:{}", grpc_port).parse()?;
    let auth_service = AuthServiceHandler::default();
    let grpc_server = Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr);

    // Initialise Http server
    println!("Starting actix_web server on port {}", actix_port.clone());
    let http_server = HttpServer::new(move || {

        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .send_wildcard()
            .supports_credentials()
            .allowed_origin("https://examclutch.com")
            .allowed_origin("https://www.examclutch.com")
            .max_age(None);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            // JWT service
            .app_data(Data::new(jwt_encoding_key.clone()))
            // Auth service
            .service(discord::redirect)
    })
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run();

    let (_, _) = tokio::join!(grpc_server, http_server);

    Ok(())
}
