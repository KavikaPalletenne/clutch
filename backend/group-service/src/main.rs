use actix_web::{App, HttpServer};
use anyhow::Result;

mod group;
mod models;
mod middleware;
mod persistence;

#[actix_web::main]
async fn main() -> Result<()> {
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");

    println!("Starting server on port 441");

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .service(group::create_group)
            .service(group::get_group_by_id)
    })
        .bind("0.0.0.0:441")?
        .run()
        .await?;

    Ok(())
}
