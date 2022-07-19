use anyhow::Result;
use meilisearch_sdk::search::SearchResults;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::{EKey, get_download_file_url, S3Bucket, SearchIndex, Database};
use crate::service::url_generation::{generate_create_resource_url, generate_discord_link_url};
use crate::resource::Resource;

use lexical_util::num::AsPrimitive;
use serenity::http::CacheHttp;
use crate::service::group::get_id_by_discord_id;
use crate::service::user::{get_user_by_discord_id, read};

// // Import commands
// use crate::commands::resource::create;
// use crate::commands::search::search;

pub struct Bot;

// General commands group (root)
#[group]
#[sub_groups(resourceg)]
#[commands(search)]
struct General;

#[group]
#[prefix = "resource"]
#[only_in("guilds")]
#[commands(create)]
struct ResourceG;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn create_discord_client(token: String, bot: Bot) -> Result<Client> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP)
        .group(&RESOURCEG_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    Ok(Client::builder(&token, intents)
        .event_handler(bot)
        .framework(framework)
        .await
        .expect("Error creating Discord client"))
}

#[command]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let group = ctx.http().get_guild(msg.guild_id.unwrap().0).await.unwrap();
    let user = msg.author.clone();

    let data = ctx.data.read().await;
    let encoding_key = data.get::<EKey>().unwrap();
    let database = data.get::<Database>().unwrap();

    let db_user = get_user_by_discord_id(user.id.to_string(), database).await?;

    if let None = db_user {
        let url = generate_discord_link_url(user, encoding_key);
        let response = format!("Your Discord account has not been linked with ExamClutch. Click the following link to link:\n{}",url);

        msg.author.create_dm_channel(ctx).await.unwrap().send_message(ctx, |m| {
            m.content(response)
                .tts(false)
            // .embed(|e| e.title("This is an embed").description("With a description"))
        }).await.unwrap();

        return Ok(())
    }

    let url = generate_create_resource_url(group, user, encoding_key, database).await;

    // TODO: Send a button with the url as a DM to the user
    let response = format!("Click the link to upload:\n{}",url);

    msg.author.create_dm_channel(ctx).await.unwrap().send_message(ctx, |m| {
        m.content(response)
            .tts(false)
        // .embed(|e| e.title("This is an embed").description("With a description"))
    }).await.unwrap();

    Ok(())
}

#[command]
pub async fn search(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let possible_group_id = msg.guild_id.clone();

    if let Some(guild_id) = possible_group_id {
        let data = ctx.data.read().await;

        let search_index = data.get::<SearchIndex>().unwrap();
        let s3_bucket = data.get::<S3Bucket>().unwrap();
        let database = data.get::<Database>().unwrap();

        let discord_group_id = guild_id.0.to_string();

        let group_id = get_id_by_discord_id(discord_group_id, database).await.unwrap();
        let user = msg.author.clone();
        let search_term = args.rest().to_string();
        let results: SearchResults<Resource> = search_index
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
            return Ok(());
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
                        get_download_file_url(
                            r.group_id.clone(),
                            r.id.clone(),
                            f.name.clone(),
                            &s3_bucket
                        )
                    )
                        .as_str(),
                )
            }
        }
        response.push_str("\n");

        msg.reply(ctx, response).await.unwrap();
    }

    Ok(())
}
