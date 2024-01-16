use reqwest::Client;
use std::collections::HashMap;
use std::hash::Hash;
use std::string::ToString;
use std::time::Duration;

use serenity::all::{ComponentInteractionDataKind, CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::{
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::framework::standard::*;
use serenity::futures::StreamExt;
use serenity::prelude::*;

use serde_json::de::Read;
use serde_json::{from_str, Value};

use crate::DnD::Convert;
use crate::DnD::Schemas::{APIReference, APIReferenceList};
use crate::DnD::{API_SERVER};
use crate::DnD::Class::ClassInfo::ClassInfo;
use crate::DnD::Class::ClassSpecific::*;
use crate::DnD::Class::ClassSpecific::SPFactory::{ClassType, SPConvert};

//This is the main struct for class level
pub struct ClassLevel {
    pub index: String,
    pub url: String,
    pub level: i32,
    pub ability_score_bonus: i32,
    pub prof_bonus: i32,
    pub feature: Vec<APIReference>,
    pub spellcasting: HashMap<String, i64>,
    pub class_specific: Box<(dyn SPConvert + Send)>,
    pub class: APIReference,
    subclass_check: bool,
    pub subclass: APIReference,
}
impl ClassLevel {
    fn new(class: &SPFactory::ClassType) -> Self {
        ClassLevel {
            index: String::new(),
            url: String::new(),
            level: -1,
            ability_score_bonus: -1,
            prof_bonus: -1,
            feature: vec![],
            spellcasting: HashMap::new(),
            class_specific: SPFactory::SPFactory::new(class),
            class: APIReference::new(),
            subclass_check: false,
            subclass: APIReference::new(),
        }
    }
}

#[async_trait]
impl Convert for ClassLevel {
    async fn from_value(&mut self, json: Value) {
        match json.get("level") {
            Some(T) => {
                self.level = T.as_i64().unwrap() as i32;
            }
            None => {print!("No level found")}
        }
        match json.get("ability_score_bonuses") {
            Some(T) => {
                self.ability_score_bonus = T.as_i64().unwrap() as i32;
            }
            None => {print!("No ability_score_bonus found")}
        }
        match json.get("prof_bonus") {
            Some(T) => {
                self.prof_bonus = T.as_i64().unwrap() as i32;
            }
            None => {print!("No prof_bonus found")}
        }
        match json.get("feature") {
            Some(T) => {
                for feature in T.as_array().unwrap() {
                    let mut temp = APIReference::new();
                    temp.from_value(feature.clone()).await;
                    self.feature.push(temp);
                }
            }
            None => {print!("No feature_choices found")}
        }
        match json.get("spellcasting") {
            Some(T) => {
                for spellcasting in T.as_object().unwrap() {
                    self.spellcasting.insert(spellcasting.0.to_string(), spellcasting.1.as_i64().unwrap());
                }
            }
            None => {print!("No spellcasting found")}
        }
        match json.get("class_specific") {
            Some(T) => {
                self.class_specific.from_value(T.clone()).await;
            }
            None => {print!("No class_specific found")}
        }
        match json.get("index") {
            Some(T) => {
                self.index = T.as_str().unwrap().to_string();
            }
            None => {print!("No index found")}
        }
        match json.get("url") {
            Some(T) => {
                self.url = T.as_str().unwrap().to_string();
            }
            None => {print!("No url found")}
        }
        match json.get("class") {
            Some(T) => {
                self.class.from_value(T.clone()).await;
            }
            None => {print!("No class found")}
        }
        match json.get("subclass") {
            Some(T) => {
                self.subclass_check = true;
                self.subclass.from_value(T.clone()).await;
            }
            None => {print!("No subclass found")}
        }
    }
}

pub async fn send_class_level_response(ctx: &Context, msg: &Message, _class: &str, _subclass: &str, _level: &str, _option: &str) -> CommandResult {
    let client = Client::new();
    println!("{}/api/classes/{}/levels{}{}{}", API_SERVER, _class, _subclass, _level, _option);
    let res = client
        .get(format!("{}/api/classes/{}/levels{}{}{}", API_SERVER, _class, _subclass, _level, _option))
        .send()
        .await
        .expect("fail to get to link")
        .text()
        .await
        .expect("fail to convert to json");
    let json: serde_json::Value = from_str(&res).expect("what?");
    let mut class_type = SPFactory::ClassType::Barbarian;
    match _class {
        "bard" => {class_type = SPFactory::ClassType::Bard;}
        "cleric" => {class_type = SPFactory::ClassType::Cleric;}
        "druid" => {class_type = SPFactory::ClassType::Druid;}
        "fighter" => {class_type = SPFactory::ClassType::Fighter;}
        "monk" => {class_type = SPFactory::ClassType::Monk;}
        "paladin" => {class_type = SPFactory::ClassType::Paladin;}
        "ranger" => {class_type = SPFactory::ClassType::Ranger;}
        "rogue" => {class_type = SPFactory::ClassType::Rogue;}
        "sorcerer" => {class_type = SPFactory::ClassType::Sorcerer;}
        "warlock" => {class_type = SPFactory::ClassType::Warlock;}
        "wizard" => {class_type = SPFactory::ClassType::Wizard;}
        _ => {}
    }
    let mut class_levels : Vec<ClassLevel> = Vec::new();
    if _level.is_empty() {
        for class_level in json.as_array().unwrap() {
            let mut temp = ClassLevel::new(&class_type);
            temp.from_value(class_level.clone()).await;
            class_levels.push(temp);
        }
    }
    else {
        let mut temp = ClassLevel::new(&class_type);
        temp.from_value(json).await;
        class_levels.push(temp);
    }

    let mut embed = CreateEmbed::new().title(format!("Class level: {}", _class).to_string());
    // Add a timestamp for the current time
    // This also accepts a rfc3339 Timestamp
    embed = embed.clone().timestamp(Timestamp::now());

    let builder = CreateMessage::new()
        .content(format!("Class level: {}", _class))
        .embed(embed);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error {:?}", why);
    }

    Ok(())
}


