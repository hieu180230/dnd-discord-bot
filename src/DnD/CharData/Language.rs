use serde_json::Value;
use serde_json::from_str;

use phf::{Map, phf_map};

use std::string::ToString;
use reqwest::{Client};

use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::async_trait;
use serenity::builder::CreateEmbed;

use crate::DnD::API_SERVER;
use crate::DnD::CharData::Convert;
use crate::DnD::Schemas::APIReference;

#[derive(Clone, Debug)]
enum LANGUAGE_TYPE{
    Standard,
    Exotic,
    None,
}

const LANGUAGE_URL: &str = "/api/languages/";


pub const LANGUAGE: &[&str] = &["abyssal", "celestial", "common", "deep-speech",
    "draconic", "dwarvish", "elvish", "giant", "gnomish", "goblin", "halfling",
    "infernal", "orc", "primordial", "sylvan", "undercommon"];

static HASH_LANGUAGE:Map<&str, LANGUAGE_TYPE> = phf_map! {
    "Standard" => LANGUAGE_TYPE::Standard,
    "Exotic" => LANGUAGE_TYPE::Exotic,
    "" => LANGUAGE_TYPE::None,
};
pub struct Language{
    pub reference:APIReference,
    pub desc:String,
    pub language_type:LANGUAGE_TYPE,
    pub script:String,
    pub typical_speaker:Vec<String>,
}
impl Language{
    fn new() -> Self{
        Language{
            reference:APIReference::new(),
            desc:"".to_string(),
            language_type:LANGUAGE_TYPE::Standard,
            script:"".to_string(),
            typical_speaker:vec![],
        }
    }
}

#[async_trait]
impl Convert for Language{
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
        match json.get("type"){
            Some(T) => {
                self.language_type = HASH_LANGUAGE.get(json["type"].as_str().unwrap()).cloned().unwrap();
            }
            None => print!("?"),
        }
        match json.get("typical_speakers"){
            Some(T) => {
                let speakers_array = T.as_array().unwrap();
                for speaker in speakers_array{
                    self.typical_speaker.push(speaker.as_str().unwrap().to_string())
                }
            }
            None => print!("?"),
        }
        match json.get("script"){
            Some(T) => {
                self.script = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
    }
}

pub async fn send_language_response(ctx: &Context, msg: &Message, lg_type:String) -> CommandResult{
    let client = Client::new();
    let res = client.get(format!("{}{}{}",API_SERVER,LANGUAGE_URL,lg_type.to_string()))
        .send()
        .await
        .expect("fail to get to link")
        .text()
        .await
        .expect("fail to convert to json");
    let json:serde_json::Value = from_str(&res).expect("what?");
    let mut a = Language::new();
    a.from_value(json.clone()).await;

    let mut speakers: String = "".to_string();
    for speaker in a.typical_speaker{
        speakers += &*format!("+ *{}*\n", speaker);
    }

    let mut embed = CreateEmbed::new()
        .title(format!("{}", a.reference.name))
        .fields(vec![("Type", format!("{:?}",a.language_type), true),
            ("Script",
             (|| -> String { if a.script == "" {return "None".to_string();} else { return a.script; }})(),
             true)])
        .field("Typical Speakers\n", speakers, false);
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

