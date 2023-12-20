use std::string::ToString;
use serde::Deserialize;
use reqwest::{Client};
use serenity::async_trait;
use crate::DnD::CharData::Convert;
use crate::DnD::Schemas::APIReference;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serde_json::from_str;
use serenity::builder::CreateEmbed;
use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::DnDCommands::str_from_vec;

const ABILITY: &str = "/api/ability-scores/";

#[derive(Deserialize)]
pub struct AbilityScore{
    pub reference:APIReference,
    pub desc:Vec<String>,
    pub full_name:String,
    pub skills:Vec<APIReference>,
}

impl AbilityScore{
    pub fn new() -> Self {
        AbilityScore{
            reference:APIReference::new(),
            desc:std::vec!["".to_string()],
            full_name: "".to_string(),
            skills: vec![APIReference::new()]
        }
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
impl Convert for AbilityScore{
    async fn from_value(&mut self, json:serde_json::Value) {
        match json.get("index"){
            Some(T) => {
                self.reference.index = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("name"){
            Some(T) => {
                self.reference.name = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("url"){
            Some(T) => {
                self.reference.url = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("full_name"){
            Some(T) => {
                self.full_name = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("desc"){
            Some(T) => {
                for i in T.as_array().unwrap(){
                    self.desc.push(i.as_str().unwrap().to_string());
                }
            },
            None => print!("?"),
        }
        match json.get("skills"){
            Some(T) => {
                for i in T.as_array().unwrap(){
                    let skill = i.as_object().unwrap();
                    let mut s = APIReference::new();
                    s.index = skill["index"].as_str().unwrap().to_string();
                    s.url = skill["url"].as_str().unwrap().to_string();
                    s.name = skill["name"].as_str().unwrap().to_string();
                    self.skills.push(s);
                }
            },
            None => print!("?"),
        }
    }
}

pub async fn send_abi_response(ctx: &Context, msg: &Message, abi_type:String) -> CommandResult{
    if abi_type != "all".to_string()
    {
        let client = Client::new();
        let res = client.get(format!("{}{}{}", API_SERVER, ABILITY, abi_type.to_string()))
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
        if a.skills.len() != 0 {
            for skill in a.skills {
                embed = embed.clone().field(format!("{}", skill.name), format!("{}", skill.url), true);
            }
        }
        if a.reference.url != "" {
            embed = embed.clone().url(format!("{}{}", API_SERVER, a.reference.url).to_string());
        }
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content("test!")
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    else
    {
        let mut embed = CreateEmbed::new()
            .title("**All available Ability Scores**");
        for i in &RESOURCES_LIST["ability-scores"].results
        {
            embed = embed.clone().field(format!("{}", i.name), format!("{}", i.index), true);
        }
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(abi_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    Ok(())
}