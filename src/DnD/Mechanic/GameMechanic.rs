use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};
use std::collections::HashMap;
use std::fmt::Display;

use lazy_static::lazy_static;
use phf::{phf_map, Map};
use reqwest::Client;
use serde_json::from_str;
use std::string::ToString;

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameMechanicType {
    Condition,
    DamageType,
    MagicSchool,
    None,
}

impl GameMechanicType {
    pub fn to_string(&self) -> String {
        match self {
            GameMechanicType::Condition => "Condition",
            GameMechanicType::DamageType => "Damage Type",
            GameMechanicType::MagicSchool => "Magic School",
            GameMechanicType::None => "None",
        }
        .to_string()
    }
    pub fn to_index(&self) -> String {
        match self {
            GameMechanicType::Condition => "conditions",
            GameMechanicType::DamageType => "damage-types",
            GameMechanicType::MagicSchool => "magic-schools",
            GameMechanicType::None => "",
        }
        .to_string()
    }
}

lazy_static! {
    pub static ref MECHANIC_DESCRIPTION: HashMap<GameMechanicType, &'static str> = {
        HashMap::from([
    (GameMechanicType::Condition, "A condition alters a creature’s capabilities in a variety of ways and can arise as a result of a spell, a class feature, a monster’s attack, or other effect. Most conditions, such as blinded, are impairments, but a few, such as invisible, can be advantageous."),
    (GameMechanicType::DamageType, "Different attacks, damaging spells, and other harmful effects deal different types of damage. Damage types have no rules of their own, but other rules, such as damage resistance, rely on the types."),
    (GameMechanicType::MagicSchool, "Academies of magic group spells into eight categories called schools of magic. Scholars, particularly wizards, apply these categories to all spells, believing that all magic functions in essentially the same way, whether it derives from rigorous study or is bestowed by a deity."),
])
    };
}

pub struct GameMechanic {
    pub gameMechanicType: GameMechanicType,
    pub index: String,
    pub name: String,
    pub url: String,
    pub desc: Vec<String>,
}

impl GameMechanic {
    pub fn new() -> Self {
        GameMechanic {
            index: String::new(),
            name: String::new(),
            url: String::new(),
            desc: vec![],
            gameMechanicType: GameMechanicType::None,
        }
    }
    pub fn set_type(&mut self, _type: GameMechanicType) {
        self.gameMechanicType = _type;
    }
}

impl Default for GameMechanic {
    fn default() -> Self {
        GameMechanic::new()
    }
}
#[async_trait]
impl Convert for GameMechanic {
    async fn from_value(&mut self, json: serde_json::Value) {
        match json.get("index") {
            Some(T) => {
                self.index = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No index found")
            }
        }
        match json.get("name") {
            Some(T) => {
                self.name = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No name found")
            }
        }
        match json.get("url") {
            Some(T) => {
                self.url = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No url found")
            }
        }
        match json.get("desc") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.desc.push(i.as_str().unwrap().to_string());
                }
            }
            None => {
                print!("No desc found")
            }
        }
    }
}

#[async_trait]
impl SendResponse for GameMechanic {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        let a_type = match _type[0] {
            "conditions" => GameMechanicType::Condition,
            "damage-types" => GameMechanicType::DamageType,
            "magic-schools" => GameMechanicType::MagicSchool,
            "all" => GameMechanicType::None,
            _ => GameMechanicType::None,
        };
        //all mechanics
        if _type[0] == "all" {
            let embed = CreateEmbed::new()
                .title("All game mechanics")
                .field(
                    format!("**{}**", GameMechanicType::Condition.to_string()),
                    format!("*{}*", &MECHANIC_DESCRIPTION[&GameMechanicType::Condition]),
                    false,
                )
                .field(
                    format!("**{}**", GameMechanicType::DamageType.to_string()),
                    format!("*{}*", &MECHANIC_DESCRIPTION[&GameMechanicType::DamageType]),
                    false,
                )
                .field(
                    format!("**{}**", GameMechanicType::MagicSchool.to_string()),
                    format!(
                        "*{}*",
                        &MECHANIC_DESCRIPTION[&GameMechanicType::MagicSchool]
                    ),
                    false,
                );
            let builder = CreateMessage::new()
                .content("All game mechanics")
                .embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        }
        //all mechanics in a type
        else if _type[1] == "all" {
            let mut embed =
                CreateEmbed::new().title(format!("All mechanics in {}", a_type.to_string()));
            for value in &RESOURCES_LIST[&a_type.to_index()].results {
                embed = embed.field(
                    format!("**{}**", value.name),
                    format!("*index: {}*", value.index),
                    true,
                );
            }
            let builder = CreateMessage::new()
                .content(format!("All mechanics in {}", a_type.to_string()))
                .embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        }
        //specific mechanic
        else {
            let client = Client::new();
            let res = client
                .get(format!("{}/api/{}/{}", API_SERVER, _type[0], _type[1]))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to text");
            let json: serde_json::Value = from_str(&res).expect("what?");
            let mut a = GameMechanic::new();
            a.from_value(json.clone()).await;
            a.set_type(a_type);

            //turn fields into string for displaying
            let mut desc = "".to_string();
            for description in &a.desc {
                desc += &*format!("- {}", description);
            }

            //create embed for displaying
            let embed = CreateEmbed::new()
                .title(format!(
                    "{}: {}",
                    a.gameMechanicType.to_string(),
                    a.name.clone()
                ))
                .description(desc);
            let builder = CreateMessage::new()
                .content(format!(
                    "{}: {}",
                    a.gameMechanicType.to_string(),
                    a.name.clone()
                ))
                .embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error {:?}", why);
            }
        }
        Ok(())
    }
}
