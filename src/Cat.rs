use crate::HELP_MESSAGE;
use reqwest::Client;
use serde::Deserialize;
use serenity::all::standard::macros;
use serenity::all::{
    CommandInteraction, CommandOptionType, Message, ResolvedOption, ResolvedValue,
};
use serenity::builder::{
    CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::framework::standard::*;
use serenity::model::Timestamp;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

///This is a slash command. Fetching an image or a fact from the API
#[derive(Deserialize)]
struct Cat {
    fact: String,
    length: u16,
}

#[derive(Deserialize)]
struct ICat {
    id: String,
    url: String,
    width: f64,
    height: f64,
}

///Fetch the Cat Image API, get the image and put it into an embed.
async fn cat_image() -> CreateEmbed {
    let client = Client::new();
    let res = client
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await
        .expect("fail to get to link")
        .json::<Vec<ICat>>()
        .await
        .expect("fail to convert to json");
    CreateEmbed::new()
        .description("Random cat :3")
        .image(&res[0].url)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now())
}

///Fetch the Cat Image API, get the fact and put it into an embed.
async fn cat_fact() -> CreateEmbed {
    let client = Client::new();
    let response = client
        .get("https://catfact.ninja/fact")
        .send()
        .await
        .expect("failed to get response")
        .json::<Cat>()
        .await
        .expect("failed to get payload");
    CreateEmbed::new()
        .title("This is a dump cat fact!")
        .description(response.fact)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now())
}

///Return a appropriate embed
pub async fn run(
    ctx: &Context,
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> CreateEmbed {
    let mut builder = CreateEmbed::new();
    if let Some(ResolvedOption {
        value: ResolvedValue::String(command),
        ..
    }) = _options.first()
    {
        println!("{}", command);
        match *command {
            "image" => {
                builder = cat_image().await;
            }
            "fact" => {
                builder = cat_fact().await;
            }
            _ => {
                builder = builder
                    .title("Hello there, Human!")
                    .description(HELP_MESSAGE);
            }
        }
    } else {
        builder = builder
            .title("Hello there, Human!")
            .description(HELP_MESSAGE);
    }
    builder
}

///Register slash command to Discord
pub fn register() -> CreateCommand {
    CreateCommand::new("cat")
        .description("A cat command")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "command",
                "thing you want about a cat",
            )
            .add_string_choice("Image", "image")
            .add_string_choice("Fact", "fact"),
        )
}
