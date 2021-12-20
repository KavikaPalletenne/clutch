use actix_web::{App, HttpServer};
use anyhow::Result;

mod group;
mod persistence;
mod user;
mod models;

#[actix_web::main]
async fn main() -> Result<()> {
    let database = persistence::create_mongodb_client()
        .await
        .expect("Failed to connect to DB");

    println!("Starting server on port 442");

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .service(user::create_user)
            .service(user::get_user_by_id)
            .service(user::user_exists)
            .service(user::update_username_by_user_id)
            .service(user::delete_user_by_id)
    })
    .bind("0.0.0.0:442")?
    .run()
    .await?;

    Ok(())
}
