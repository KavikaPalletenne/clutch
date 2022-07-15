use crate::service::url_generation::{
    generate_create_resource_url, generate_delete_url, generate_edit_url,
};
use crate::type_maps::EKey;
use jsonwebtoken::EncodingKey;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::Database;

#[command]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let group = ctx.http.get_guild(msg.guild_id.unwrap().0).await.unwrap();
    let user = msg.author.clone();

    let data = ctx.data.read().await;
    let encoding_key = data.get::<EKey>().unwrap();
    let database = data.get::<Database>().unwrap();

    let url = generate_create_resource_url(group, user, encoding_key, database).await;

    // TODO: Send a button with the url as a DM to the user
    let response = format!("Click the link:\n{}",url);

    msg.author.create_dm_channel(ctx).await.unwrap().send_message(ctx, |m| {
        m.content(response)
            .tts(false)
            // .embed(|e| e.title("This is an embed").description("With a description"))
    }).await.unwrap();

    Ok(())
}

// #[command]
// pub async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
//     let user = msg.author.clone();
//     let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();
//
//     let url = generate_delete_url(user, encoding_key);
//
//     // TODO: Send a button with the url as a DM to the user
//
//     todo!()
// }

// #[command]
// pub async fn edit(ctx: &Context, msg: &Message) -> CommandResult {
//     let user = msg.author.clone();
//     let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();
//
//     let url = generate_edit_url(user, encoding_key);
//
//     // TODO: Send a button with the url as a DM to the user
//     todo!(0)
// }
