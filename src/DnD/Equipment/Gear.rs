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
use crate::DnD::Equipment::EquipmentInfo::{Cost, Range, SendEquipmentResponse};

const EQUIPMENT: &str = "/api/equipment/";

pub struct EquipmentGear {
    pub info: APIReference,
    pub desc: Vec<String>,
    pub equipment_category: APIReference,
    pub gear_category: APIReference,
    pub cost: Option<Cost>,
    pub weight: i64,
}
impl EquipmentGear {
    pub fn new() -> Self {
        EquipmentGear {
            info: APIReference::new(),
            desc: vec![],
            equipment_category: APIReference::new(),
            gear_category: APIReference::new(),
            cost: None,
            weight: 0,
        }
    }
}

#[async_trait]
impl Convert for EquipmentGear {
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
        match json.get("weight") {
            Some(T) => {
                self.weight = T.as_i64().unwrap();
            }
            None => {println!("No weight")}
        }
    }
}

#[async_trait]
impl SendEquipmentResponse for EquipmentGear {
    async fn send_equipment_response(ctx: &Context, msg: &Message, json: Value) -> CommandResult {
        let mut gear = EquipmentGear::new();
        gear.from_value(json).await;

        let mut description: String = "".to_string();
        for desc in &gear.desc {
            description += &*format!("*{}*\n", desc);
        }

        let mut embed = CreateEmbed::new()
            .title(format!("{}: {}\n{} (Cost: {} {})",gear.equipment_category.name.clone(), gear.info.name.clone(), gear.gear_category.name.clone(), gear.cost.unwrap().get().0.clone(), gear.cost.unwrap().get().1.clone()))
            .description(description);
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new().content("test!").embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
        Ok(())
    }
}