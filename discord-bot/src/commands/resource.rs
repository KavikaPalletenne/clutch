use jsonwebtoken::EncodingKey;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::service::url_generation::{generate_create_resource_url, generate_delete_url, generate_edit_url};

#[command]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let group = msg.guild(ctx).unwrap();
    let user = msg.author.clone();
    let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();

    let url = generate_create_resource_url(group, user, encoding_key);

    // TODO: Send a button with the url as a DM to the user

    todo!()
}

#[command]
pub async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
    let user = msg.author.clone();
    let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();

    let url = generate_delete_url(user, encoding_key);

    // TODO: Send a button with the url as a DM to the user

    todo!()
}

#[command]
pub async fn edit(ctx: &Context, msg: &Message) -> CommandResult {
    let user = msg.author.clone();
    let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();

    let url = generate_edit_url(user, encoding_key);

    // TODO: Send a button with the url as a DM to the user

    todo!(0)
}
