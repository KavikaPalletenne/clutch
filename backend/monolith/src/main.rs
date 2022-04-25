use actix_web::{App, HttpServer};
use actix_cors::Cors;
// use rustls::internal::pemfile::{certs, pkcs8_private_keys};
// use rustls::{ServerConfig};
use anyhow::Result;
use jsonwebtoken::EncodingKey;
use std::env;
use actix_web::middleware::Logger;
// use std::fs::File;
// use std::io::BufReader;
use actix_web::web::Data;
use crate::storage::init_bucket;

mod oauth2;
mod models;
mod jwt;
mod resource;
mod persistence;
mod middleware;
mod group;
mod user;
mod shared;
mod cdn;
// mod file;
mod storage;
mod search;


#[actix_web::main]
async fn main() -> Result<()> {
    let actix_port = std::env::var("ACTIX_PORT").expect("Error getting ACTIX_PORT").to_string();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // // // XPS file location
    // // let cert_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/excl-api/keys/cert.pem").unwrap());
    // // let key_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/excl-api/keys/key.pem").unwrap());
    // //
    // // PC file location
    // let cert_file = &mut BufReader::new(File::open("C:/Users/User/Documents/Development/GitHub/clutch/backend/keys/cert.pem").unwrap());
    // let key_file = &mut BufReader::new(File::open("C:/Users/User/Documents/Development/GitHub/clutch/backend/keys/key.pem").unwrap());
    //
    // let cert_chain = certs(cert_file).unwrap();
    // let mut keys = pkcs8_private_keys(key_file).unwrap();
    // if keys.is_empty() {
    //     eprintln!("Could not locate PKCS 8 private keys.");
    //     std::process::exit(1);
    // }
    //
    // // load ssl keys
    // let mut config = ServerConfig::builder()
    //     .with_safe_defaults()
    //     .with_no_client_auth()
    //     .with_single_cert(cert_chain, keys.remove(0))
    //     .expect("bad certificate/key");
    // // ServerConfig::new(NoClientAuth::new());


    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key("./keys/key.pem")
    //     .unwrap();
    // builder.set_certificate("./keys/cert.pem").unwrap();

    // Initialise JWT settings
    let jwt_secret = env::var("JWT_SECRET").expect("Error getting JWT_SECRET").to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());

    // Initialise DB
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");
    println!("Successfully connected to database");

    // Initialise Meilisearch Connection
    let search_endpoint = env::var("SEARCH_ENDPOINT").expect("Error getting SEARCH_ENDPOINT").to_string();
    let search_index = meilisearch_sdk::client::Client::new(search_endpoint, "masterKey").index("resources");
    search_index.set_filterable_attributes(["group_id", "subject", "tags"]).await.unwrap();

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
            // OAuth2 Service
            .app_data(Data::new(jwt_encoding_key.clone()))
            .service(oauth2::user_registration)
            .service(oauth2::authorize)
            .service(oauth2::get_user_guilds)
            // Resource Service
            .app_data(Data::new(database.clone()))
            .service(resource::create_resource)
            .service(resource::fetch_resource_by_id)
            .service(resource::fetch_resource_by_group_id)
            .service(resource::update_resource)
            .service(resource::delete_resource)
            // Group Service
            .service(group::create_group)
            .service(group::get_group_by_id)
            .service(group::get_group_name_by_id)
            .service(group::join_group)
            .service(group::leave_group)
            // User Service
            .service(user::create_user)
            .service(user::get_user_by_id)
            .service(user::get_username_by_id)
            .service(user::user_exists)
            .service(user::update_username_by_user_id)
            .service(user::update_email_by_user_id)
            .service(user::delete_user_by_id)
            .service(user::get_user_groups)
            //CDN
            .app_data(Data::new(bucket.clone()))
            .service(cdn::download_file)
            // Easter Eggs
            .service(shared::easter_egg)
            // Search
            .app_data(Data::new(search_index.clone()))
            .service(search::search)
            .service(search::search_blank)
    })
        // .bind_openssl("0.0.0.0:443", builder)?
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run()
        .await?;

    Ok(())
}