use std::error::Error;
use std::marker::Send;
use std::sync::Arc;
use crate::{I_HELP_COMMAND, HELP_MESSAGE};
use serenity::all::{Message, MessageBuilder, Ready};
use serenity::async_trait;
use serenity::prelude::*;
use serenity::all::standard::macros;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::framework::standard::*;
use serenity::gateway::ShardManager;

pub struct Handler;
pub struct ShardManagerContainer; //this is to contain shard managers
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

#[macros::command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult{
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;
    Ok(())
}
#[macros::command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult{
    let channel = match msg.channel_id.to_channel(&ctx).await{
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return Err(Box::new(why) as Box<dyn Error + Send + Sync>);
        },
    };
    let resp = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name) //this make the text italic or bold, ...
        .push(" ping the channel ")
        .mention(&channel)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &resp).await{
        println!("Error sending ping msg: {:?}",why);
    }
    return Ok(());
}
#[macros::command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult{
    let data = ctx.data.read().await;

    let shardmanager = match data.get::<ShardManagerContainer>(){
        Some(v) => v,
        None =>{
            msg.reply(ctx, "problem getting shard manager")
                .await.expect("problem sending error message");
            return Ok(());
        },
    };

    let runners = shardmanager.runners.lock().await;

    let runner = match runners.get(&ctx.shard_id){
        Some(runner) => runner,
        None => {
            msg.reply(&ctx, "No shard found")
                .await.expect("problem sending error message");
            return Ok(());
        }
    };

    msg.reply(&ctx, &format!("Latency: {:?}", runner.latency.unwrap()))
        .await.expect("fail to send latency");
    Ok(())
}
#[macros::group]
#[commands(help, ping, latency, some_long_command)]
struct Manager;
#[async_trait]
impl EventHandler for Handler {
     async fn ready(&self, _: Context, ready: Ready) {
         println!("{} is ready!", ready.user.name);
     }
}

#[macros::command]
async fn some_long_command(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, &format!("Arguments: {:?}", args.rest())).await?;

    Ok(())
}
