use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use reqwest::Client;
use serde_json::{from_str, Value};
use std::string::ToString;

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

pub struct Skill {
    pub reference: APIReference,
    pub desc: Vec<String>,
    pub ability_score: APIReference,
}

const SKILL_URL: &str = "/api/skills/";

impl Skill {
    pub fn new() -> Self {
        Skill {
            reference: APIReference::new(),
            desc: vec![],
            ability_score: APIReference::new(),
        }
    }
}
impl Default for Skill {
    fn default() -> Self {
        Skill::new()
    }
}
///get skill's instance from a json serde_json::Value
#[async_trait]
impl Convert for Skill {
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
                for desc in T.as_array().unwrap() {
                    self.desc.push(desc.as_str().unwrap().to_string());
                }
            }
            None => print!("?"),
        }
        match json.get("ability_score") {
            Some(T) => {
                self.ability_score = APIReference::parse(T);
            }
            None => print!("?"),
        }
    }
}

#[async_trait]
impl SendResponse for Skill {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        if _type[0] != "all" {
            let client = Client::new();
            let res = client
                .get(format!("{}{}{}", API_SERVER, SKILL_URL, _type[0]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = Skill::new();
            a.from_value(json.clone()).await;

            let mut description: String = "".to_string();
            for desc in &a.desc {
                description += &*format!("*{}*\n", desc);
            }

            let mut embed = CreateEmbed::new()
                .title(a.reference.name.clone())
                .description(description)
                .field(
                    format!("Ability Score ({})", a.ability_score.name),
                    format!("{}{}", API_SERVER, a.ability_score.url),
                    false,
                );

            if !a.reference.url.is_empty() {
                embed = embed
                    .clone()
                    .url(format!("{}{}", API_SERVER, a.reference.url).to_string());
            }
            // Add a timestamp for the current time
            // This also accepts a rfc3339 Timestamp
            embed = embed.clone().timestamp(Timestamp::now());
            let builder = CreateMessage::new().content(_type[0]).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        } else {
            let mut embed = CreateEmbed::new().title("**All available Skills**");
            for i in &RESOURCES_LIST["skills"].results {
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
