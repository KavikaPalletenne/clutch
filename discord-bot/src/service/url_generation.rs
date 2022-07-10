use jsonwebtoken::EncodingKey;
use serenity::model::prelude::User;
use serenity::utils::Guild;
use crate::service::jwt::create_create_resource_token;

pub fn generate_create_resource_url(group_id: i64, user: User, encoding_key: &EncodingKey) -> String {
    let username = user.name;
    let user_id = user.id.0.as_i64();
    let avatar_hash = user.avatar.unwrap();

    let group_id = guild.id.0.as_i64();

    let token = create_create_resource_token(
        user_id,
        group_id,
        username,
        avatar_hash,
        encoding_key,
    );

    format!("https://examclutch.com/discord/create?token={}", token)
}

pub fn generate_delete_url(group_id: i64, user: User, encoding_key: &EncodingKey) -> String {
    todo!()
}

pub fn generate_edit_url() -> String {
    todo!()
}
