use anyhow::Result;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

pub(crate) struct Bot;

// General commands group
#[group]
struct General;

impl EventHandler for Bot {}

pub async fn create_discord_client(token: String, bot: &Bot) -> Result<Client> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    Ok(Client::builder(&token, intents)
        .event_handler(bot)
        .framework(framework)
        .await.expect("Error creating Discord client"))
}
