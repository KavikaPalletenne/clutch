use anyhow::Result;
use meilisearch_sdk::search::SearchResults;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::cdn::get_download_file_url;
use crate::resource::Resource;

mod persistence;
mod search;
mod storage;
mod cdn;
mod resource;

struct Bot {
    database: mongodb::Database,
    search_index: meilisearch_sdk::indexes::Index,
    s3_bucket: s3::Bucket,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0.to_string();
        let group_id = msg.guild_id.unwrap().to_string();
        // TODO: Switch to embeds
        if let Some(task) = msg.content.strip_prefix("$resource search") {
            let search_term = task.trim().to_string();
            println!("Received request for search - Term: {}", search_term.clone());
            let results: SearchResults<Resource> = self.search_index.search()
                .with_query(&search_term)
                .with_filter(&*format!("group_id = {}", group_id))
                .execute()
                .await
                .unwrap();

            let mut resources = Vec::<Resource>::new();
            for hit in results.hits.iter() {
                resources.push(hit.result.clone());
            }

            let mut response = format!("{} Search results for \"{}\":\n", msg.author.mention(), search_term.clone());

            if resources.len() == 0 {
                msg.reply(ctx, format!("No resources found for \"{}\"", search_term).as_str()).await.unwrap();
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
                        response.push_str(format!("\t{}: {}", f.name.clone(), get_download_file_url(
                        r.id.clone(),
                          f.name.clone(),
                            &self.s3_bucket)).as_str()
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
    let token = std::env::var("DISCORD_TOKEN").expect("No Discord token found in environment");

    // Configure MongoDB connection
    let database_url = std::env::var("DATABASE_URL").expect("No database url found in environment");
    let database = persistence::create_mongodb_client(database_url).await.expect("Failed to connect to database");
    println!("Successfully connected to database");

    // Configure Meilisearch connection
    let search_url = std::env::var("SEARCH_URL").expect("No search url found in environment");
    let search_index = search::create_search_client(search_url).await.expect("Failed to connect to search server");
    println!("Successfully connected to Meilisearch");

    // Configure object storage connection
    let bucket_name = std::env::var("S3_BUCKET_NAME").expect("S3 env variable error");
    let access_key = std::env::var("S3_ACCESS_KEY").expect("S3 env variable error");
    let secret_key = std::env::var("S3_SECRET_KEY").expect("S3 env variable error");
    let region_name = std::env::var("S3_REGION_NAME").expect("S3 env variable error");
    let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3 env variable error");
    let s3_bucket = storage::create_s3_bucket(
        bucket_name,
        access_key,
        secret_key,
        region_name,
        s3_endpoint
    );
    println!("Successfully configured object storage");

    let bot = Bot {
        database,
        search_index,
        s3_bucket,
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = // TODO: Implement command framework or split commands into different files atleast - follow this https://github.com/serenity-rs/serenity/tree/current/examples/e06_sample_bot_structure or https://github.com/serenity-rs/serenity/tree/current/examples/e05_command_framework or even slash commands
        Client::builder(&token, intents).event_handler(bot).await.expect("Error creating Discord client");
    client.start().await.unwrap();
}
