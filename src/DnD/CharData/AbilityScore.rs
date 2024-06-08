use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use std::string::ToString;

use crate::DnD::DnDCommands::str_from_vec;
use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

const ABILITY: &str = "/api/ability-scores/";

#[derive(Deserialize)]
pub struct AbilityScore {
    pub reference: APIReference,
    pub desc: Vec<String>,
    pub full_name: String,
    pub skills: Vec<APIReference>,
}

impl AbilityScore {
    pub fn new() -> Self {
        AbilityScore {
            reference: APIReference::new(),
            desc: std::vec!["".to_string()],
            full_name: "".to_string(),
            skills: vec![APIReference::new()],
        }
    }
}
impl Default for AbilityScore {
    fn default() -> Self {
        AbilityScore::new()
    }
}

// Custom struct to hold alias-link pairs
pub struct AbilityScoreAlias {
    pub alias: &'static str,
    pub link: &'static str,
}

// Vec to store the alias-link pairs
pub const ABILITY_SCORE_ALIASES: &[AbilityScoreAlias] = &[
    AbilityScoreAlias {
        alias: "charisma",
        link: "cha",
    },
    AbilityScoreAlias {
        alias: "constitution",
        link: "con",
    },
    AbilityScoreAlias {
        alias: "dexterity",
        link: "dex",
    },
    AbilityScoreAlias {
        alias: "intelligence",
        link: "int",
    },
    AbilityScoreAlias {
        alias: "strength",
        link: "str",
    },
    AbilityScoreAlias {
        alias: "wisdom",
        link: "wis",
    },
];

#[async_trait]
impl Convert for AbilityScore {
    async fn from_value(&mut self, json: serde_json::Value) {
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
        match json.get("full_name") {
            Some(T) => {
                self.full_name = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("desc") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.desc.push(i.as_str().unwrap().to_string());
                }
            }
            None => print!("?"),
        }
        match json.get("skills") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    let skill = i.as_object().unwrap();
                    let mut s = APIReference::new();
                    s.index = skill["index"].as_str().unwrap().to_string();
                    s.url = skill["url"].as_str().unwrap().to_string();
                    s.name = skill["name"].as_str().unwrap().to_string();
                    self.skills.push(s);
                }
            }
            None => print!("?"),
        }
    }
}

#[async_trait]
impl SendResponse for AbilityScore {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        if _type[0] != "all" {
            let client = Client::new();
            let res = client
                .get(format!("{}{}{}", API_SERVER, ABILITY, _type[0]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = AbilityScore::new();
            a.from_value(json.clone()).await;
            let mut embed = CreateEmbed::new()
                .title(format!("{}/{}", a.reference.name, a.full_name))
                .description(str_from_vec(a.desc).await);
            if !a.skills.is_empty() {
                for skill in a.skills {
                    embed = embed
                        .clone()
                        .field(skill.name.clone(), skill.url.clone(), true);
                }
            }
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
            let mut embed = CreateEmbed::new().title("**All available Ability Scores**");
            for i in &RESOURCES_LIST["ability-scores"].results {
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
