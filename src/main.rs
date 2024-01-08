#![allow(
    unused_variables,
    unused_mut,
    dead_code,
    unused_imports,
    unused_labels,
    non_snake_case
)]
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::*
};
use discord_bot::Handler::{Handler, MANAGER_GROUP, ShardManagerContainer};
use discord_bot::DnD::{DnDCommands, RESOURCES_LIST};
use discord_bot::DnD::DnDCommands::{DND_GROUP};
use discord_bot::DnD::Schemas::APIReferenceList;


use std::collections::HashMap;



#[tokio::main]
async fn main() {
    //init the resource list
    if !RESOURCES_LIST.is_empty(){println!("ok loading data");}

    //discord bot token
    //load environment variable from .env file
    dotenv::dotenv().expect("Failed to load .env file");
    //initiate the logger to use environment variable
    tracing_subscriber::fmt::init(); //need to read document

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let framework : StandardFramework = StandardFramework::new()
        .group(&MANAGER_GROUP)
        .group(&DND_GROUP);
    framework.configure(Configuration::new().prefix("!"));

    //this build the client with Handler and frame
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    //clone the shard manager and will shut down along with main program
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
