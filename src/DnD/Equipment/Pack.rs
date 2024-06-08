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

pub struct EquipmentPack {
    pub info: APIReference,
    pub desc: Vec<String>,
    pub equipment_category: APIReference,
    pub gear_category: APIReference,
    pub cost: Option<Cost>,
    pub contents: Vec<APIReference>,
}
impl EquipmentPack {
    pub fn new() -> Self {
        EquipmentPack {
            info: APIReference::new(),
            desc: vec![],
            equipment_category: APIReference::new(),
            gear_category: APIReference::new(),
            cost: None,
            contents: vec![],
        }
    }
}

#[async_trait]
impl Convert for EquipmentPack {
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
        match json.get("gear_category") {
            Some(T) => {
                self.gear_category.from_value(T.clone()).await;
            }
            None => {println!("No gear_category")}
        }
        match json.get("cost") {
            Some(T) => {
                self.cost = Some(Cost::new(T["quantity"].as_i64().unwrap(), T["unit"].as_str().unwrap().to_string()));
            }
            None => {println!("No cost")}
        }
        match json.get("contents") {
            Some(T) => {
                for content in T.as_array().unwrap() {
                    let mut api_ref = APIReference::new();
                    api_ref.from_value(content.clone()).await;
                    self.contents.push(api_ref);
                }
            }
            None => {println!("No contents")}
        }
    }
}