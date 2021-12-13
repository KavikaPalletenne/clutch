use actix_web::{App, HttpServer};
use anyhow::Result;
use clap::Parser;

mod models;
mod persistence;
mod resource;

#[actix_web::main]
async fn main() -> Result<()> {
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");

    println!("Starting server on port 443");

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .service(resource::create_resource)
            .service(resource::fetch_resource_by_id)
            .service(resource::fetch_resource_by_group_id)
            .service(resource::update_resource)
            .service(resource::delete_resource)
    })
    .bind("0.0.0.0:443")?
    .run()
    .await?;

    Ok(())
}

// #[derive(Parser, Debug)]
// #[clap(version = concat!(env!("CARGO_PKG_VERSION")), about = "Open-source studying built for students.")]
// struct Args {
//     #[clap(
//         short,
//         long,
//         about = "Address to bind the server to.",
//         env,
//         default_value = "0.0.0.0"
//     )]
//     address: String,
//     #[clap(short, long, about = "Port to listen on.", env, default_value = "8080")]
//     port: u16,
// }
