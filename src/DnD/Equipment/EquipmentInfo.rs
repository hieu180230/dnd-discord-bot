use reqwest::Client;
use serde::Deserialize;
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::string::ToString;

use crate::DnD::DnDCommands::str_from_vec;
use crate::DnD::Schemas::{APIReference, Damage};
use crate::DnD::{Convert, SendResponse};
use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Equipment::*;

use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::*;
use serenity::prelude::*;


const EQUIPMENT: &str = "/api/equipment/";

pub struct Range {
    normal: i64,
    long: i64,
}
impl Range {
    pub fn new(r_normal: i64, r_long: i64) -> Self {
        Range {
            normal: r_normal,
            long: r_long,
        }
    }
    pub fn set(&mut self, r_normal: i64, r_long: i64) {
        self.normal = r_normal;
        self.long = r_long;
    }
    pub fn get(&self) -> (i64, i64) {
        (self.normal, self.long)
    }
}
pub struct Cost {
    pub quantity: i64,
    pub unit: String,
}
impl Cost {
    pub fn new(c_quantity: i64, c_unit: String) -> Self {
        Cost {
            quantity: c_quantity,
            unit: c_unit,
        }
    }
    pub fn set(&mut self, c_quantity: i64, c_unit: String) {
        self.quantity = c_quantity;
        self.unit = c_unit;
    }
    pub fn get(&self) -> (i64, String) {
        (self.quantity, self.unit.clone())
    }
}

#[async_trait]
pub trait SendEquipmentResponse {
    async fn send_equipment_response(ctx: &Context, msg: &Message, json: serde_json::Value) -> CommandResult;
}

struct EquipmentInfo;
#[async_trait]
impl SendResponse for EquipmentInfo {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        let client = Client::new();
        let res = client
            .get(format!("{}{}{}", API_SERVER, EQUIPMENT, _type[0]))
            .send()
            .await
            .expect("fail to get to link")
            .text()
            .await
            .expect("fail to convert to json");
        let json: serde_json::Value = from_str(&res).expect("what?");
        let mut equipment_type = "gear".to_string();
        for (key, value) in json.as_object().unwrap() {
            if key == "contents" {
                equipment_type = "pack".to_string();
                break;
            }
            if key == "weapon_category" {
                equipment_type = "weapon".to_string();
                break;
            }
            if key == "armor_category" {
                equipment_type = "armor".to_string();
                break;
            }
        }
        match &*equipment_type {
            "gear" => {
                //response gear type message
            }
            "weapon" => {
                //response weapon type message
                Weapon::EquipmentWeapon::send_equipment_response(ctx, msg, json).await.expect("fail to send weapon response");
            }
            "armor" => {
                //response armor type message
            }
            "pack" => {
                //response pack type message
            }
            _ => {println!("fail matching type");}
        }
        Ok(())
    }
}

