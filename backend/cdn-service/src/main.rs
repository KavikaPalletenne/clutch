use anyhow::Result;
use actix_web::{HttpServer, App};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use jsonwebtoken::{DecodingKey, EncodingKey};
use crate::storage::init_bucket;

mod cdn;
mod persistence;
mod models;
mod middleware;
mod jwt;
mod storage;
mod group;


#[tokio::main]
async fn main() -> Result<()> { // Result<(), Box<dyn Error>>
    let actix_port = std::env::var("ACTIX_PORT").expect("Error getting ACTIX_PORT").to_string();

    // // Initialise logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Initialised logger");

    // Initialise JWT settings
    let jwt_secret = std::env::var("JWT_SECRET").expect("Error getting JWT_SECRET").to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes().clone());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    println!("Initialised JWT settings");

    // Initialise DB connection
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");
    println!("Successfully connected to database");

    // Initialise S3 Bucket
    let bucket = init_bucket();

    // Initialise Http server
    println!("Starting actix_web server on port {}", actix_port.clone());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .send_wildcard()
            .supports_credentials()
            .allowed_origin("http://127.0.0.1")
            .allowed_origin("https://127.0.0.1")
            .allowed_origin("https://examclutch.com")
            .allowed_origin("https://www.examclutch.com")
            .max_age(None);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            // JWT service
            .app_data(Data::new(jwt_encoding_key.clone()))
            .app_data(Data::new(jwt_decoding_key.clone()))
            //CDN
            .app_data(Data::new(database.clone()))
            .app_data(Data::new(bucket.clone()))
            .service(cdn::download_file)
    })
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run()
        .await?;

    Ok(())
}
