use jsonwebtoken::EncodingKey;
use serenity::model::prelude::User;
use serenity::utils::Guild;
use crate::service::jwt::{generate_create_resource_token, generate_user_token};

pub fn generate_create_resource_url(group: Guild, user: User, encoding_key: &EncodingKey) -> String {
    let group_id = group.id.0.as_i64();
    let group_name = group.name;
    let username = user.name;
    let user_id = user.id.0.as_i64();
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
    let user_id = user.id.0.as_i64();
    let avatar_hash = user.avatar.unwraph();

    let token = generate_user_token(
        user_id,
        username,
        avatar_hash,
        encoding_key
    );

    format!("https://examclutch.com/discord/delete?token={}", token)
}

pub fn generate_edit_url(user: User, encoding_key: &EncodingKey) -> String {
    let username = user.name;
    let user_id = user.id.0.as_i64();
    let avatar_hash = user.avatar.unwraph();

    let token = generate_user_token(
        user_id,
        username,
        avatar_hash,
        encoding_key
    );

    format!("https://examclutch.com/discord/edit?token={}", token)
}
