use anyhow::Result;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn create_mongodb_client() -> Result<Database> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let client_options = ClientOptions::parse(database_url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("ExamClutch");

    Ok(database)
}
