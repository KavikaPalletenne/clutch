use anyhow::Result;
use actix_web::{ HttpServer, App };
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use jsonwebtoken::{DecodingKey, EncodingKey};
use crate::persistence::create_mongodb_client;

mod search;
mod models;
mod persistence;
mod group;
mod jwt;
mod middleware;

#[actix_web::main]
async fn main() -> Result<()> {
    let actix_port = std::env::var("ACTIX_PORT").expect("Error getting ACTIX_PORT").to_string();

    // Initialise logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Initialised logger");

    // Initialise JWT settings
    let jwt_secret = std::env::var("JWT_SECRET").expect("Error getting JWT_SECRET").to_string();
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    println!("Initialised JWT settings");

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

    // Initialise MongoDB connection
    let database = create_mongodb_client()
        .await
        .expect("Failed to connect to DB");
    println!("Successfully connected to database");

    // Start Http server
    println!("Starting actix_web server on port {}", actix_port.clone());
    HttpServer::new(move || {

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
            .app_data(Data::new(database.clone()))
            // JWT service
            .app_data(Data::new(jwt_encoding_key.clone()))
            .app_data(Data::new(jwt_decoding_key.clone()))
            // Search service
            .app_data(Data::new(search_index.clone()))
            .service(search::search)
            .service(search::search_blank)
    })
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run()
        .await?;

    Ok(())
}
