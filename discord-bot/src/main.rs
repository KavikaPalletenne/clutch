use crate::bot::{Bot, create_discord_client};
use crate::cdn::get_download_file_url;
use crate::resource::Resource;
use crate::storage::init_bucket;
use anyhow::Result;
use meilisearch_sdk::search::SearchResults;
use sea_orm::{ConnectOptions, Database};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serenity::Error::Decode;
use crate::persistence::create_db_client;

mod bot;
mod cdn;
mod persistence;
mod resource;
mod search;
mod storage;
mod commands;
mod service;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0.to_string();
        let group_id = msg.guild_id.unwrap().to_string();
        // TODO: Switch to embeds
        if let Some(task) = msg.content.strip_prefix("$resource search") {
            let search_term = task.trim().to_string();
            println!(
                "Received request for search - Term: {}",
                search_term.clone()
            );
            let results: SearchResults<Resource> = self
                .search_index
                .search()
                .with_query(&search_term)
                .with_filter(&*format!("group_id = {}", group_id))
                .execute()
                .await
                .unwrap();

            let mut resources = Vec::<Resource>::new();
            for hit in results.hits.iter() {
                resources.push(hit.result.clone());
            }

            let mut response = format!(
                "{} Search results for \"{}\":\n",
                msg.author.mention(),
                search_term.clone()
            );

            if resources.len() == 0 {
                msg.reply(
                    ctx,
                    format!("No resources found for \"{}\"", search_term).as_str(),
                )
                .await
                .unwrap();
                return;
            }

            let r = &resources[0];
            response.push_str(format!("Title: {}", r.title).as_str());
            response.push_str("\n");
            response.push_str(format!("Description: {}", r.description).as_str());
            response.push_str("\n");
            if let Some(files) = &r.files {
                response.push_str("Files:\n");
                for f in files {
                    response.push_str(
                        format!(
                            "\t{}: {}",
                            f.name.clone(),
                            get_download_file_url(r.id.clone(), f.name.clone(), &self.s3_bucket)
                        )
                        .as_str(),
                    )
                }
            }
            response.push_str("\n");

            msg.reply(ctx, response).await.unwrap();
        }
    }
}

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
        .expect("Error getting ACTIX_PORT")
        .to_string();
    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .expect("Error getting ACTIX_PORT")
        .to_string()
        .parse::<u32>()
        .unwrap();
    let database = create_db_client(db_url, max_connections);

    // Initialise S3 Bucket
    let s3_bucket = init_bucket();

    let bot = Bot;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = create_discord_client(token, &bot)
        .await.expect("Error creating Discord client");

    { // Insert data
        let mut data = client.data.write().await;
        data.insert::<sea_orm::DatabaseConnection>(database);
        data.insert::<meilisearch_sdk::indexes::Index>(search_index);
        data.insert::<s3::Bucket>(s3_bucket);
        data.insert::<EncodingKey>(jwt_encoding_key);
        data.insert::<DecodingKey>(jwt_decoding_key);
    }

    client.start().await.unwrap();
}
