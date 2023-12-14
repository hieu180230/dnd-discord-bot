use serde_json::Value;
use crate::DnD::CharData::Convert;
use crate::DnD::Schemas::APIReference;
use std::string::ToString;
use reqwest::{Client};
use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serde_json::from_str;
use serenity::builder::CreateEmbed;
use crate::DnD::API_SERVER;

pub struct Alignment{
    reference:APIReference,
    desc:String,
    abbreviation:String,
}

pub const ALIGNMENTS: &[&str] = &["chaotic-neutral", "chaotic-evil" , "chaotic-good" ,
                                       "lawful-neutral" , "lawful-evil" , "lawful-good" ,
                                       "neutral" , "neutral-evil" , "neutral-good"];

impl Alignment{
    pub fn new() -> Self{
        Alignment{
            reference:APIReference::new(),
            desc:"".to_string(),
            abbreviation:"".to_string(),
        }
    }
}
const ALIGNMENT_LINK: &str = "/api/alignments/";
#[async_trait]
impl Convert for Alignment{
    async fn from_value(&mut self, json: Value) {
        match json.get("index"){
            Some(T) => {
                self.reference.index = json["index"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("name"){
            Some(T) => {
                self.reference.name = json["name"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("url"){
            Some(T) => {
                self.reference.url = json["url"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("desc"){
            Some(T) => {
                self.desc = json["desc"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("abbreviation"){
            Some(T) => {
                self.abbreviation = json["abbreviation"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
    }
}

pub async fn send_alignment_response(ctx: &Context, msg: &Message, ali_type:String) -> CommandResult{
    let client = Client::new();
    let res = client.get(format!("{}{}{}",API_SERVER,ALIGNMENT_LINK,ali_type.to_string()))
        .send()
        .await
        .expect("fail to get to link")
        .text()
        .await
        .expect("fail to convert to json");
    let json:serde_json::Value = from_str(&res).expect("what?");
    let mut a = Alignment::new();
    a.from_value(json.clone()).await;
    let mut embed = CreateEmbed::new()
        .title(format!("{}/{}",a.reference.name, a.abbreviation))
        .description(a.desc);
    if a.reference.url != ""{
        embed = embed.clone().url(format!("{}{}",API_SERVER, a.reference.url).to_string());
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
    Ok(())
}