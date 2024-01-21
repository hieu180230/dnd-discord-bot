use serde_json::from_str;
use serde_json::Value;

use phf::{phf_map, Map};

use reqwest::Client;
use std::string::ToString;

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

#[derive(Clone, Debug)]
enum LANGUAGE_TYPE {
    Standard,
    Exotic,
    None,
}

const LANGUAGE_URL: &str = "/api/languages/";

pub const LANGUAGE: &[&str] = &[
    "abyssal",
    "celestial",
    "common",
    "deep-speech",
    "draconic",
    "dwarvish",
    "elvish",
    "giant",
    "gnomish",
    "goblin",
    "halfling",
    "infernal",
    "orc",
    "primordial",
    "sylvan",
    "undercommon",
];

static HASH_LANGUAGE: Map<&str, LANGUAGE_TYPE> = phf_map! {
    "Standard" => LANGUAGE_TYPE::Standard,
    "Exotic" => LANGUAGE_TYPE::Exotic,
    "" => LANGUAGE_TYPE::None,
};
pub struct Language {
    pub reference: APIReference,
    pub desc: String,
    pub language_type: LANGUAGE_TYPE,
    pub script: String,
    pub typical_speaker: Vec<String>,
}
impl Language {
    fn new() -> Self {
        Language {
            reference: APIReference::new(),
            desc: "".to_string(),
            language_type: LANGUAGE_TYPE::Standard,
            script: "".to_string(),
            typical_speaker: vec![],
        }
    }
}

#[async_trait]
impl Convert for Language {
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
        match json.get("type") {
            Some(T) => {
                self.language_type = HASH_LANGUAGE.get(T.as_str().unwrap()).cloned().unwrap();
            }
            None => print!("?"),
        }
        match json.get("typical_speakers") {
            Some(T) => {
                let speakers_array = T.as_array().unwrap();
                for speaker in speakers_array {
                    self.typical_speaker
                        .push(speaker.as_str().unwrap().to_string())
                }
            }
            None => print!("?"),
        }
        match json.get("script") {
            Some(T) => {
                self.script = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
    }
}

#[async_trait]
impl SendResponse for Language {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        if _type[0] != "all".to_string() {
            let client = Client::new();
            let res = client
                .get(format!("{}{}{}", API_SERVER, LANGUAGE_URL, _type[0]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = Language::new();
            a.from_value(json.clone()).await;

            let mut speakers: String = "".to_string();
            for speaker in a.typical_speaker {
                speakers += &*format!("+ *{}*\n", speaker);
            }

            let mut embed = CreateEmbed::new()
                .title(format!("{}", a.reference.name))
                .fields(vec![
                    ("Type", format!("{:?}", a.language_type), true),
                    (
                        "Script",
                        (|| -> String {
                            if a.script == "" {
                                return "None".to_string();
                            } else {
                                return a.script;
                            }
                        })(),
                        true,
                    ),
                ])
                .field("Typical Speakers\n", speakers, false);
            if a.reference.url != "" {
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
            let mut embed = CreateEmbed::new().title("**All available Languages**");
            for i in &RESOURCES_LIST["languages"].results {
                embed = embed
                    .clone()
                    .field(format!("{}", i.name), format!("{}", i.index), true);
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
