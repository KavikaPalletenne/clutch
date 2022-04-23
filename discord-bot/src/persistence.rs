use anyhow::Result;
use mongodb::{
    options::{ClientOptions},
    Client,
    Database,
};

pub async fn create_mongodb_client(url: String) -> Result<Database> {
    let client_options = ClientOptions::parse(url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("ExamClutch");

    Ok(database)
}
