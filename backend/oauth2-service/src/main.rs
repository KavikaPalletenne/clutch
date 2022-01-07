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


#[actix_web::main]
async fn main() -> Result<()> {

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/oauth2-service/keys/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("C:/Users/kbpal/Documents/Development/clutch/backend/oauth2-service/keys/key.pem").unwrap());
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


    println!("Starting server on port 443.");
    HttpServer::new(move || {
        App::new()
            .data(jwt_encoding_key.clone())
            .service(oauth2::user_registration)
            .service(oauth2::authorize)
            .service(oauth2::get_user_guilds)
    })
        .bind_rustls("0.0.0.0:443", config)?
        .run()
        .await?;

    Ok(())
}
