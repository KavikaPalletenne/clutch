use std::time::Duration;
use anyhow::Result;
use mongodb::{options::ClientOptions, Client, Database};
use sea_orm::{ConnectOptions, DatabaseConnection};

pub async fn create_mongodb_client(url: String) -> Result<Database> {
    let client_options = ClientOptions::parse(url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("ExamClutch");

    Ok(database)
}

pub fn create_db_client(url: String, max_connections: u32) -> DatabaseConnection {
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(max_connections)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    Database::connect(options).await?
}
