use crate::DnD::Schemas::APIReference;
use std::string::ToString;
use reqwest::{Client};
use serenity::async_trait;
use crate::DnD::CharData::Convert;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serde_json::{from_str, Value};
use serenity::builder::CreateEmbed;
use crate::DnD::{API_SERVER, RESOURCES_LIST};

pub struct Skill{
    pub reference: APIReference,
    pub desc:Vec<String>,
    pub ability_score:APIReference,
}

const SKILL_URL: &str = "/api/skills/";

impl Skill{
    pub fn new() -> Self{
        Skill{
            reference:APIReference::new(),
            desc:vec![],
            ability_score:APIReference::new(),
        }
    }
}

#[async_trait]
impl Convert for Skill{
    async fn from_value(&mut self, json: Value) {
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
        match json.get("desc"){
            Some(T) => {
                for desc in T.as_array().unwrap(){
                    self.desc.push(desc.as_str().unwrap().to_string());
                }
            }
            None => print!("?"),
        }
        match json.get("ability_score"){
            Some(T) => {
                self.ability_score = APIReference::parse(T);
            }
            None => print!("?"),
        }
    }
}

pub async fn send_skill_response(ctx: &Context, msg: &Message, prf_type:String) -> CommandResult{
    if prf_type != "all".to_string()
    {
        let client = Client::new();
        let res = client.get(format!("{}{}{}", API_SERVER, SKILL_URL, prf_type.to_string()))
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
            .title(format!("{}", a.reference.name))
            .description(description)
            .field(format!("Ability Score ({})", a.ability_score.name),
                   format!("{}{}", API_SERVER, a.ability_score.url), false);

        if a.reference.url != "" {
            embed = embed.clone().url(format!("{}{}", API_SERVER, a.reference.url).to_string());
        }
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(prf_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    else
    {
        let mut embed = CreateEmbed::new()
            .title("**All available Skills**");
        for i in &RESOURCES_LIST["skills"].results
        {
            embed = embed.clone().field(format!("{}", i.name), format!("{}", i.index), true);
        }
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(prf_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    Ok(())
}