use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use reqwest::Client;
use serde_json::from_str;
use std::string::ToString;

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

const PROFICIENCIES_URL: &str = "/api/proficiencies/";

pub struct Proficiencies {
    pub reference: APIReference,
    pub proficiencies_type: String,
    pub classes: Vec<APIReference>,
    pub races: Vec<APIReference>,
    pub references: APIReference,
}

impl Proficiencies {
    pub fn new() -> Self {
        Proficiencies {
            reference: APIReference::new(),
            proficiencies_type: "".to_string(),
            classes: vec![],
            races: vec![],
            references: APIReference::new(),
        }
    }
}
impl Default for Proficiencies {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Convert for Proficiencies {
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
        match json.get("type") {
            Some(T) => {
                self.proficiencies_type = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("classes") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.classes.push(APIReference {
                        index: i["index"].as_str().unwrap().to_string(),
                        name: i["name"].as_str().unwrap().to_string(),
                        url: i["url"].as_str().unwrap().to_string(),
                    })
                }
            }
            None => print!("?"),
        }
        match json.get("races") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.races.push(APIReference {
                        index: i["index"].as_str().unwrap().to_string(),
                        name: i["name"].as_str().unwrap().to_string(),
                        url: i["url"].as_str().unwrap().to_string(),
                    })
                }
            }
            None => print!("?"),
        }
        match json.get("reference") {
            Some(T) => {
                let i = T.as_object().unwrap();
                self.references = APIReference {
                    index: i["index"].as_str().unwrap().to_string(),
                    name: i["name"].as_str().unwrap().to_string(),
                    url: i["url"].as_str().unwrap().to_string(),
                }
            }
            None => print!("?"),
        }
    }
}

#[async_trait]
impl SendResponse for Proficiencies {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        if _type[0] != "all" {
            let client = Client::new();
            let res = client
                .get(format!("{}{}{}", API_SERVER, PROFICIENCIES_URL, _type[0]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = Proficiencies::new();
            a.from_value(json.clone()).await;

            //turn fields into strings fro displaying
            let mut races: String = "".to_string();
            let mut classes: String = "".to_string();
            for race in &a.races {
                races += &*format!("+ *{}*\n", race.name);
            }
            for class in &a.classes {
                classes += &*format!("+ *{}*\n", class.name);
            }

            let mut embed = CreateEmbed::new()
                .title(a.reference.name.clone())
                .field("Type", a.proficiencies_type.clone(), false)
                .fields(vec![
                    (
                        "Classes",
                        {
                            if a.classes.is_empty() {
                                "None".to_string()
                            } else {
                                classes
                            }
                        },
                        true,
                    ),
                    (
                        "Races",
                        {
                            if a.races.is_empty() {
                                "None".to_string()
                            } else {
                                races
                            }
                        },
                        true,
                    ),
                ]);
            if !a.references.url.is_empty() {
                embed = embed
                    .clone()
                    .url(format!("{}{}", API_SERVER, a.references.url).to_string());
            }
            // Add a timestamp for the current time
            // This also accepts a rfc3339 Timestamp
            embed = embed.clone().timestamp(Timestamp::now());
            let builder = CreateMessage::new().content(_type[0]).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        } else {
            let mut embed = CreateEmbed::new().title("**All available Proficiencies**");
            for i in &RESOURCES_LIST["proficiencies"].results {
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
