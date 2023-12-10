use serde::Deserialize;
use reqwest::{Client};
use serenity::all::{Message};
use serenity::prelude::*;
use serenity::all::standard::macros;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::Timestamp;
use serenity::framework::standard::*;

#[derive(Deserialize)]
struct CAT{
    fact : String,
    length: u16
}

#[derive(Deserialize)]
struct ICAT{
    id : String,
    url : String,
    width : f64,
    height : f64,
}

#[macros::group]
#[prefix(cat)]
#[only_in(guilds)]
#[default_command(cat_help)]
#[commands(cat_image, cat_fact)]
#[summary = "This is all you need if you are a cat lover\n"]
struct Cat;

#[macros::command]
#[aliases(image)]
async fn cat_image(ctx: &Context, msg: &Message) -> CommandResult{
    let client = Client::new();
    let res = client.get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await
        .expect("fail to get to link")
        .json::<Vec<ICAT>>()
        .await
        .expect("fail to convert to json");
    let embed = CreateEmbed::new()
        .description("Random cat :3")
        .image(&res[0].url)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now());
    let builder = CreateMessage::new()
        .embed(embed);
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;
    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    }
    Ok(())
}
#[macros::command]
#[aliases(fact)]
async fn cat_fact(ctx: &Context, msg: &Message) -> CommandResult{
    let client = Client::new();
    let response = client.get("https://catfact.ninja/fact")
        .send()
        .await
        .expect("failed to get response")
        .json::<CAT>()
        .await
        .expect("failed to get payload");
    if let Err(why) = msg.channel_id.say(&ctx.http, response.fact).await {
        println!("Error {:?}", why);
    }
    Ok(())
}
#[macros::command]
#[aliases(help)]
async fn cat_help(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let saying = if args.is_empty(){
        "Commands that you can use are:\n!cat fact\n!cat image\n".to_string()
    } else {
        format!("This group does not have {}", args.rest())
    };
    msg.channel_id.say(&ctx.http, saying).await.expect("could not send help");
    Ok(())
}