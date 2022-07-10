use jsonwebtoken::EncodingKey;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::service::url_generation::{generate_create_resource_url, generate_delete_url, generate_edit_url};

#[command]
pub async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    let group_id = msg.guild_id.unwrap().0.as_i64().clone(); // TODO: See what can be done about this (id might not fit into i64 as it is a u64)
    let user = msg.author.clone();
    let encoding_key = ctx.data.read().await.get::<EncodingKey>().unwrap();

    let url = generate_create_resource_url(group_id, user, encoding_key);

    // TODO: Send an embed with the url as a DM

    todo!()
}

pub async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = msg.author.id.0.as_i64();

    let url = generate_delete_url(user_id);

    // TODO

    todo!()
}

pub async fn edit(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = msg.author.id.0.as_i64();

    let url = generate_edit_url(user_id);

    // TODO

    todo!(0)
}
