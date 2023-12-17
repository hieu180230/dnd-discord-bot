#![allow(
    unused_variables,
    unused_mut,
    dead_code,
    unused_imports,
    unused_labels,
    non_snake_case
)]
use std::env;
use std::sync::Arc;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::*
};
use discord_bot::Cat::CAT_GROUP;
use discord_bot::Handler::{Handler, MANAGER_GROUP, ShardManagerContainer};
use discord_bot::DnD::DnDCommands;
use discord_bot::DnD::DnDCommands::DND_GROUP;


#[tokio::main]
async fn main() {
    //discord bot token
    env::set_var("DISCORD_TOKEN", "MTE4MDgxMzM0MTk0NjU2MDU0Mg.Gesusw.IpfDTthqaJGcjMyVc_ow5yjSUuJYAkfkn7l7Ls");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let framework : StandardFramework = StandardFramework::new()
        .group(&MANAGER_GROUP)
        .group(&CAT_GROUP)
        .group(&DND_GROUP);
    framework.configure(Configuration::new().prefix("!"));

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}