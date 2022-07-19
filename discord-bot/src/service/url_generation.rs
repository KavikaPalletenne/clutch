use crate::service::jwt::{generate_create_resource_token, generate_user_token};
use jsonwebtoken::EncodingKey;
use lexical_util::num::AsPrimitive;
use sea_orm::DatabaseConnection;
use serenity::model::guild::{Guild, PartialGuild};
use serenity::model::prelude::User;
use crate::service::group::get_id_by_discord_id;

pub async fn generate_create_resource_url(
    group: PartialGuild,
    user: User,
    encoding_key: &EncodingKey,
    conn: &DatabaseConnection,
) -> String {
    let discord_group_id = group.id.to_string();

    let group_id = get_id_by_discord_id(discord_group_id, conn).await.unwrap();
    let group_name = group.name;
    let username = user.name;
    let user_id = user.id.to_string();
    let avatar_hash = user.avatar.unwrap();

    let token = generate_create_resource_token(
        user_id,
        group_id,
        group_name,
        username,
        avatar_hash,
        encoding_key,
    );

    format!("https://examclutch.com/discord/create?token={}", token)
}

pub fn generate_delete_url(user: User, encoding_key: &EncodingKey) -> String {
    let username = user.name;
    let user_id = user.id.to_string();
    let avatar_hash = user.avatar.unwrap();

    let token = generate_user_token(user_id, username, avatar_hash, encoding_key);

    format!("https://examclutch.com/discord/delete?token={}", token)
}

pub fn generate_edit_url(user: User, encoding_key: &EncodingKey) -> String {
    let username = user.name;
    let user_id = user.id.to_string();
    let avatar_hash = user.avatar.unwrap();

    let token = generate_user_token(user_id, username, avatar_hash, encoding_key);

    format!("https://examclutch.com/discord/edit?token={}", token)
}

pub fn generate_discord_link_url(user: User, encoding_key: &EncodingKey) -> String {
    let username = user.name;
    let user_id = user.id.to_string();
    let avatar_hash = user.avatar.unwrap();

    let token = generate_user_token(user_id, username, avatar_hash, encoding_key);

    format!("https://examclutch.com/discord/link?token={}", token)
}
