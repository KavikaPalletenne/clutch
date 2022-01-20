use actix_web::{App, HttpServer};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use anyhow::Result;
use jsonwebtoken::EncodingKey;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod oauth2;
mod models;
mod jwt;
mod resource;
mod persistence;
mod middleware;
mod group;
mod user;


#[actix_web::main]
async fn main() -> Result<()> {

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/excl-api/keys/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/excl-api/keys/key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // Initialise JWT settings
    let jwt_secret = env::var("JWT_SECRET").expect("Error getting JWT_SECRET").to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());

    // Initialise DB
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");
    println!("Successfully connected to database");

    println!("Starting server on port 443.");
    HttpServer::new(move || {
        App::new()
            // OAuth2 Service
            .data(jwt_encoding_key.clone())
            .service(oauth2::user_registration)
            .service(oauth2::authorize)
            .service(oauth2::get_user_guilds)
            // Resource Service
            .data(database.clone())
            .service(resource::create_resource)
            .service(resource::fetch_resource_by_id)
            .service(resource::fetch_resource_by_group_id)
            .service(resource::update_resource)
            .service(resource::delete_resource)
            // Group Service
            .service(group::create_group)
            .service(group::get_group_by_id)
            // User Service
            .service(user::create_user)
            .service(user::get_user_by_id)
            .service(user::user_exists)
            .service(user::update_username_by_user_id)
            .service(user::update_email_by_user_id)
            .service(user::delete_user_by_id)
    })
        .bind_rustls("0.0.0.0:443", config)?
        .run()
        .await?;

    Ok(())
}
