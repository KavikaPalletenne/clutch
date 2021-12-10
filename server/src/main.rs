use actix_web::{App, HttpServer};
use anyhow::Result;
use clap::Parser;

mod persistence;
mod models;
mod resource;

#[actix_web::main]
async fn main() -> Result<()> {

    let args = Args::parse();
    let address = format!("{}:{}", args.address, args.port);
    let database = persistence::create_mongodb_client().await.expect("Failed to connect to DB");

    println!("Starting server on port {}", args.port);

    HttpServer::new( move ||
        App::new()
            .data(database.clone())
            .service(resource::create_resource)
            .service(resource::update_resource)
            .service(resource::update_resource2)
    )
        .bind(address)?
        .run()
        .await?;

    Ok(())
}

#[derive(Parser)]
#[clap(version = concat!(env!("CARGO_PKG_VERSION")), about = "Open-source studying built for students.")]
struct Args {
    #[clap(
        short,
        long,
        about = "Address to bind the server to.",
        env,
        default_value = "0.0.0.0"
    )]
    address: String,
    #[clap(short, long, about = "Port to listen on.", env, default_value = "8080")]
    port: u16,
}