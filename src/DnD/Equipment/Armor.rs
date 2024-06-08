use reqwest::Client;
use serde::Deserialize;
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::string::ToString;

use crate::DnD::DnDCommands::str_from_vec;
use crate::DnD::Schemas::{APIReference, Damage};
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;
use crate::DnD::Equipment::EquipmentInfo::{Cost, Range};

const EQUIPMENT: &str = "/api/equipment/";

pub struct EquipmentArmor {
    pub info: APIReference,
    pub desc: Vec<String>,
    pub equipment_category: APIReference,
    pub armor_category: String,
    pub armor_class: HashMap<String, String>,
    pub str_minimum: i64,
    pub stealth_disadvantage: bool,
    pub cost: Option<Cost>,
    pub weight: i64,
}
impl EquipmentArmor {
    pub fn new() -> Self {
        EquipmentArmor {
            info: APIReference::new(),
            desc: vec![],
            equipment_category: APIReference::new(),
            armor_category: "".to_string(),
            armor_class: HashMap::new(),
            str_minimum: 0,
            stealth_disadvantage: false,
            cost: None,
            weight: 0,
        }
    }
}

#[async_trait]
impl Convert for EquipmentArmor {
    async fn from_value(&mut self, json: Value) {
        match json.get("index") {
            Some(T) => {
                self.info.from_value(T.clone()).await;
            }
            None => {println!("No index")}
        }
        match json.get("desc") {
            Some(T) => {
                for description in T.as_array().unwrap() {
                    self.desc.push(description.as_str().unwrap().to_string());
                }
            }
            None => {println!("No desc")}
        }
        match json.get("equipment_category") {
            Some(T) => {
                self.equipment_category.from_value(T.clone()).await;
            }
            None => {println!("No equipment_category")}
        }
        match json.get("armor_category") {
            Some(T) => {
                self.armor_category = T.to_string();
            }
            None => {println!("No armor_category")}
        }
        match json.get("armor_class") {
            Some(T) => {
                for (key, value) in T.as_object().unwrap() {
                    self.armor_class.insert(key.to_string(), value.as_str().unwrap().to_string());
                }
            }
            None => {println!("No armor_class")}
        }
        match json.get("str_minimum") {
            Some(T) => {
                self.str_minimum = T.as_i64().unwrap();
            }
            None => {println!("No str_minimum")}
        }
        match json.get("stealth_disadvantage") {
            Some(T) => {
                self.stealth_disadvantage = T.as_bool().unwrap();
            }
            None => {println!("No stealth_disadvantage")}
        }
        match json.get("cost") {
            Some(T) => {
                self.cost = Some(Cost::new(T["quantity"].as_i64().unwrap(), T["unit"].as_str().unwrap().to_string()));
            }
            None => {println!("No cost")}
        }
        match json.get("weight") {
            Some(T) => {
                self.weight = T.as_i64().unwrap();
            }
            None => {println!("No weight")}
        }
    }
}