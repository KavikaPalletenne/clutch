use anyhow::Result;
use actix_web::{HttpServer, App};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use jsonwebtoken::{DecodingKey, EncodingKey};
use crate::storage::init_bucket;

mod resource;
mod models;
mod persistence;
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

    // Initialise Meilisearch connection
    let search_endpoint = std::env::var("SEARCH_ENDPOINT").expect("Error getting SEARCH_ENDPOINT").to_string();
    let search_api_key = std::env::var("SEARCH_API_KEY").expect("Error getting SEARCH_API_KEY").to_string();
    let search_index = meilisearch_sdk::client::Client::new(search_endpoint, search_api_key).index("resources");
    println!("Initialised Meilisearch connection");
    let search_attributes = vec![
        "group_id",
        "subject",
        "tags"
    ];
    search_index.set_filterable_attributes(search_attributes.clone()).await.unwrap();
    println!("Added search filterable attributes: {:?}", search_attributes);

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
            // Resource Service
            .app_data(Data::new(database.clone()))
            .app_data(Data::new(search_index.clone()))
            .app_data(Data::new(bucket.clone()))
            .service(resource::create_resource)
            .service(resource::fetch_resource_by_id)
            .service(resource::fetch_resource_by_group_id)
            .service(resource::update_resource)
            .service(resource::delete_resource)
    })
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run()
        .await?;

    Ok(())
}
