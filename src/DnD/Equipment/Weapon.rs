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

pub struct EquipmentWeapon {
    pub info: APIReference,
    pub desc: Vec<String>,
    pub equipment_category: APIReference,
    pub weapon_category: String,
    pub weapon_range: String,
    pub category_range: String,
    pub range: Option<Range>,
    pub damage: Option<Damage>,
    pub two_handed_damage: Option<Damage>,
    pub properties: Vec<APIReference>,
    pub cost: Option<Cost>,
    pub weight: i64,
}
impl EquipmentWeapon {
    pub fn new() -> Self {
        EquipmentWeapon {
            info: APIReference::new(),
            desc: vec![],
            equipment_category: APIReference::new(),
            weapon_category: "".to_string(),
            weapon_range: "".to_string(),
            category_range: "".to_string(),
            range: None,
            damage: None,
            two_handed_damage: None,
            properties: vec![],
            cost: None,
            weight: 0,
        }
    }
}
#[async_trait]
impl Convert for EquipmentWeapon {
    async fn from_value(&mut self, json: Value) {
        self.info = APIReference::parse(&json);
        match json.get("desc") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.desc.push(i.as_str().unwrap().to_string());
                }
            }
            None => {
                println!("No desc");
            }
        }
        match json.get("equipment_category") {
            Some(T) => {
                self.equipment_category = APIReference::parse(T);
            }
            None => {
                println!("No equipment_category");
            }
        }
        match json.get("weapon_category") {
            Some(T) => {
                self.weapon_category = T.as_str().unwrap().to_string();
            }
            None => {
                println!("No weapon_category");
            }
        }
        match json.get("weapon_range") {
            Some(T) => {
                self.weapon_range = T.as_str().unwrap().to_string();
            }
            None => {
                println!("No weapon_range");
            }
        }
        match json.get("category_range") {
            Some(T) => {
                self.category_range = T.as_str().unwrap().to_string();
            }
            None => {
                println!("No category_range");
            }
        }
        match json.get("range") {
            Some(T) => {
                self.range = Some(Range::new(
                    T["normal"].as_i64().unwrap(),
                    T["long"].as_i64().unwrap(),
                ));
            }
            None => {
                println!("No range");
            }
        }
        match json.get("damage") {
            Some(T) => {
                let mut damage = Damage::new();
                damage.from_value(T.clone()).await;
                self.damage = Some(damage);
            }
            None => {
                println!("No damage");
            }
        }
        match json.get("two_handed_damage") {
            Some(T) => {
                let mut two_handed_damage = Damage::new();
                two_handed_damage.from_value(T.clone()).await;
                self.two_handed_damage = Some(two_handed_damage);
            }
            None => {
                println!("No two_handed_damage");
            }
        }
        match json.get("properties") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.properties.push(APIReference::parse(i));
                }
            }
            None => {
                println!("No properties");
            }
        }
        match json.get("cost") {
            Some(T) => {
                self.cost = Some(Cost::new(
                    T["quantity"].as_i64().unwrap(),
                    T["unit"].as_str().unwrap().to_string(),
                ));
            }
            None => {
                println!("No cost");
            }
        }
        match json.get("weight") {
            Some(T) => {
                self.weight = T.as_i64().unwrap();
            }
            None => {
                println!("No weight");
            }
        }
    }
}

#[async_trait]
impl SendEquipmentResponse for EquipmentWeapon {
    async fn send_equipment_response(ctx: &Context, msg: &Message, json: Value) -> CommandResult {
        let mut weapon = EquipmentWeapon::new();
        weapon.from_value(json).await;

        let mut description: String = "".to_string();
        for desc in &weapon.desc {
            description += &*format!("*{}*\n", desc);
        }

        let mut embed = CreateEmbed::new()
            .title(format!("{}: {}\n{} (Cost: {} {})",weapon.equipment_category.name.clone(), weapon.info.name.clone(), weapon.category_range.clone(), weapon.cost.as_ref().unwrap().get().0.clone(), weapon.cost.as_ref().unwrap().get().1.clone()))
            .description(description);
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new().content("test!").embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
        Ok(())
    }
}