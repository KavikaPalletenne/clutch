use anyhow::Result;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn create_db_client(url: String, max_connections: u32) -> Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(max_connections)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    Ok(Database::connect(options).await?)
}
