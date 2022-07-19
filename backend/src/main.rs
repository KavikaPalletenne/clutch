use crate::handler::{cdn, easter_egg, group, resource, search, user};
use crate::service::storage::init_bucket;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::time::Duration;

mod auth;
mod errors;
mod handler;
mod models;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
    let actix_port = std::env::var("ACTIX_PORT")
        .expect("Error getting ACTIX_PORT")
        .to_string();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialise JWT settings
    let jwt_secret = env::var("JWT_SECRET")
        .expect("Error getting JWT_SECRET")
        .to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    // Initialise DB Connection
    let db_url = std::env::var("DATABASE_URL")
        .expect("Error getting ACTIX_PORT")
        .to_string();
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .expect("Error getting ACTIX_PORT")
        .to_string()
        .parse::<u32>()
        .unwrap();
    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(max_connections)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let conn = Database::connect(options).await?;

    // Initialise Meilisearch Connection
    let search_endpoint = env::var("SEARCH_ENDPOINT")
        .expect("Error getting SEARCH_ENDPOINT")
        .to_string();
    let search_index =
        meilisearch_sdk::client::Client::new(search_endpoint, "masterKey").index("resources");
    // search_index
    //     .set_filterable_attributes(["group_id", "subject", "tags"])
    //     .await
    //     .unwrap();

    // Initialise S3 Bucket
    let bucket = init_bucket();

    println!("Starting server on port {}.", actix_port.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            // .allow_any_origin()
            .send_wildcard()
            .supports_credentials()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://127.0.0.1")
            .allowed_origin("http://examclutch.com")
            .allowed_origin("http://www.examclutch.com")
            .allowed_origin("https://examclutch.com")
            .allowed_origin("https://www.examclutch.com")
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().ends_with(b".localhost")
            // })
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .max_age(None);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            // Auth service
            .app_data(Data::new(jwt_encoding_key.clone()))
            .app_data(Data::new(jwt_decoding_key.clone()))
            .service(handler::auth::register)
            .service(handler::auth::login)
            .service(handler::auth::discord_link)
            .service(handler::auth::authorize)
            // OAuth2 Service
            // .service(oauth2::user_registration)
            // .service(oauth2::authorize)
            // .service(oauth2::get_user_guilds) // TODO: Oauth2 Service
            // Resource Service
            .app_data(Data::new(conn.clone()))
            .service(resource::create_resource)
            .service(resource::discord_create_resource)
            .service(resource::get)
            .service(resource::get_by_group)
            .service(resource::delete_resource)
            // .service(resource::update_resource)
            // .service(resource::delete_resource)
            // Group Service
            .service(group::create_group)
            .service(group::get)
            .service(group::get_name)
            .service(group::join_group)
            .service(group::leave_group)
            .service(group::get_user_groups)
            // User Service
            .service(user::create_user)
            .service(user::get)
            .service(user::get_username)
            .service(user::update)
            .service(user::delete)
            .service(user::check_email)
            .service(user::check_username)
            //CDN
            .app_data(Data::new(bucket.clone()))
            .service(cdn::download_file)
            // Easter Eggs
            .service(easter_egg::easter_egg)
            // Search
            .app_data(Data::new(search_index.clone()))
            .service(search::search)
            .service(search::search_blank)
    })
    .bind(format!("0.0.0.0:{}", actix_port))?
    .run()
    .await?;

    Ok(())
}
