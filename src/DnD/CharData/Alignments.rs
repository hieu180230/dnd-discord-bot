use reqwest::Client;
use serde_json::from_str;
use serde_json::Value;
use std::string::ToString;

use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

pub struct Alignment {
    reference: APIReference,
    desc: String,
    abbreviation: String,
}

pub const ALIGNMENTS: &[&str] = &[
    "chaotic-neutral",
    "chaotic-evil",
    "chaotic-good",
    "lawful-neutral",
    "lawful-evil",
    "lawful-good",
    "neutral",
    "neutral-evil",
    "neutral-good",
];

impl Alignment {
    pub fn new() -> Self {
        Alignment {
            reference: APIReference::new(),
            desc: "".to_string(),
            abbreviation: "".to_string(),
        }
    }
}
impl Default for Alignment {
    fn default() -> Self {
        Alignment::new()
    }
}

const ALIGNMENT_LINK: &str = "/api/alignments/";
#[async_trait]
impl Convert for Alignment {
    async fn from_value(&mut self, json: Value) {
        match json.get("index") {
            Some(T) => {
                self.reference.index = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("name") {
            Some(T) => {
                self.reference.name = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("url") {
            Some(T) => {
                self.reference.url = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("desc") {
            Some(T) => {
                self.desc = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("abbreviation") {
            Some(T) => {
                self.abbreviation = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
    }
}

#[async_trait]
impl SendResponse for Alignment {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        if _type[0] != "all" {
            let client = Client::new();
            let res = client
                .get(format!("{}{}{}", API_SERVER, ALIGNMENT_LINK, _type[0]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = Alignment::new();
            a.from_value(json.clone()).await;
            let mut embed = CreateEmbed::new()
                .title(format!("{}/{}", a.reference.name, a.abbreviation))
                .description(a.desc);
            if !a.reference.url.is_empty() {
                embed = embed
                    .clone()
                    .url(format!("{}{}", API_SERVER, a.reference.url).to_string());
            }
            // Add a timestamp for the current time
            // This also accepts a rfc3339 Timestamp
            embed = embed.clone().timestamp(Timestamp::now());
            let builder = CreateMessage::new().content("test!").embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        } else {
            let mut embed = CreateEmbed::new().title("**All available Alignments**");
            for i in &RESOURCES_LIST["alignments"].results {
                embed = embed.clone().field(i.name.clone(), i.index.clone(), true);
            }
            embed = embed.clone().timestamp(Timestamp::now());
            let builder = CreateMessage::new().content(_type[0]).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        }
        Ok(())
    }
}
