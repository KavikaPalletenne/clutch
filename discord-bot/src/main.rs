use crate::bot::{create_discord_client, Bot};
use crate::cdn::get_download_file_url;
use crate::persistence::create_db_client;
use crate::resource::Resource;
use crate::storage::init_bucket;
use crate::type_maps::*;
use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};
use meilisearch_sdk::search::SearchResults;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Error::Decode;
use std::time::Duration;

mod bot;
mod cdn;
// mod commands;
mod persistence;
mod resource;
mod search;
mod service;
mod storage;
mod type_maps;

#[tokio::main]
async fn main() {
    // Configure client with Discord bot token
    let token = std::env::var("DISCORD_TOKEN").expect("No Discord bot token found in environment");

    // Initialise JWT settings
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("Error getting JWT_SECRET")
        .to_string();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    // Initialise Meilisearch Connection
    let search_endpoint = std::env::var("SEARCH_ENDPOINT")
        .expect("Error getting SEARCH_ENDPOINT")
        .to_string();
    let search_api_key = std::env::var("SEARCH_API_KEY")
        .expect("Error getting SEARCH_API_KEY")
        .to_string();
    let search_index =
        meilisearch_sdk::client::Client::new(search_endpoint, search_api_key).index("resources");
    // TODO: Add a command to build command or something to add filterable attributes the first time (must have atleast one resource).

    // Initialise DB Connection
    let db_url = std::env::var("DATABASE_URL")
        .expect("Error getting DATABASE_URL")
        .to_string();
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .expect("Error getting DB_MAX_CONNECTIONS")
        .to_string()
        .parse::<u32>()
        .unwrap();
    let database = create_db_client(db_url, max_connections)
        .await
        .expect("Error connecting to database");

    // Initialise S3 Bucket
    let s3_bucket = init_bucket();

    let bot = Bot;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = create_discord_client(token, bot)
        .await
        .expect("Error creating Discord client");

    {
        // Insert data
        let mut data = client.data.write().await;
        data.insert::<Database>(database);
        data.insert::<SearchIndex>(search_index);
        data.insert::<S3Bucket>(s3_bucket);
        data.insert::<EKey>(jwt_encoding_key);
        data.insert::<DKey>(jwt_decoding_key);
    }

    client.start().await.unwrap();
}
