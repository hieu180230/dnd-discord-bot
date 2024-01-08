use std::collections::HashMap;
use std::fmt::format;
use std::string::ToString;
use reqwest::{Client};

use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use serde_json::{from_str, Value};

use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Schemas::{APIReference, Choice};
use crate::DnD::CharData::{ Background};
use crate::DnD::{Convert};

pub struct SpellCasting{
    pub level:i64,
    pub info:Vec<Background::Feature>,
    pub spellcasting_ability:APIReference,
}

impl SpellCasting{
    pub fn new() -> Self{
        SpellCasting{
            level:-1,
            info:vec![],
            spellcasting_ability:APIReference::new(),
        }
    }
}

#[async_trait]
impl Convert for SpellCasting{
    async fn from_value(&mut self, json: Value) {
        match json.get("level"){
            Some(T) => {
                self.level = T.as_i64().unwrap();
            }
            None => print!("?"),
        }
        match json.get("spellcasting_ability"){
            Some(T) => {
                self.spellcasting_ability = APIReference::parse(T);
            }
            None => print!("?"),
        }
        match json.get("info"){
            Some(T) => {
                let info_array = T.as_array().unwrap();
                for info in info_array{
                    let mut descriptions: Vec<String> = vec![];
                    for desc in info["desc"].as_array().unwrap(){
                        descriptions.push(desc.as_str().unwrap().to_string());
                    }
                    let feature = Background::Feature
                    {
                        feature_type:info["name"].as_str().unwrap().to_string(),
                        desc: descriptions
                    };
                    self.info.push(feature);
                }
            }
            None => print!("?"),
        }
    }
}

impl SpellCasting{
    pub async fn display(&self) -> String{
        let mut res = "".to_string();
        //level
        if self.level != -1{
            res += &*format!("**Level**: {}\n", self.level);
        }
        //spellcasting ability
        res += &*format!("**Ability**: {}\n", self.spellcasting_ability.name);
        //If no spellcasting for this class
        if self.level == -1 && self.spellcasting_ability.name == ""{
            res = "**None**".to_string();
        }
        res
    }
}