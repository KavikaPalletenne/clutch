use anyhow::Result;
use actix_web::{HttpServer, App, Responder, get};
use actix_cors::Cors;
use actix_web::middleware::Logger;


mod easter_egg;


#[tokio::main]
async fn main() -> Result<()> { // Result<(), Box<dyn Error>>
    let actix_port = std::env::var("ACTIX_PORT").expect("Error getting ACTIX_PORT").to_string();

    // // Initialise logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("Initialised logger");


    // Initialise Http server
    println!("Starting actix_web server on port {}", actix_port.clone());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()

            .send_wildcard()
            .supports_credentials()
            .max_age(None);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(easter_egg::easter_egg)
    })
        .bind(format!("0.0.0.0:{}", actix_port))?
        .run()
        .await?;

    Ok(())
}
