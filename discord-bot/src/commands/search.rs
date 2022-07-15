use crate::type_maps::{S3Bucket, SearchIndex};
use crate::{Database, get_download_file_url, Resource};
use lexical_util::num::AsPrimitive;
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::SearchResults;
use s3::Bucket;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::service::group::get_id_by_discord_id;

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
