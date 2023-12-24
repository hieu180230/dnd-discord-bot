use std::collections::HashMap;
use serde_json::Value;
use crate::DnD::CharData::Convert;
use crate::DnD::Schemas::*;
use crate::DnD::Schemas::APIReference;
use std::string::ToString;
use reqwest::{Client};
use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serde_json::from_str;
use serenity::builder::CreateEmbed;
use crate::DnD::{API_SERVER, RESOURCES_LIST};


pub struct Feature{
    pub feature_type:String,
    pub desc:Vec<String>,
}
impl Feature{
    pub fn new() -> Self{
        Feature{
            feature_type:"".to_string(),
            desc:vec![],
        }
    }
}

const BACKGROUND_LINK:&str = "/api/backgrounds/";
pub const BACKGROUND: &[&str] = &["acolyte"];

pub struct Background{
    pub reference:APIReference,
    pub starting_proficiencies:Vec<APIReference>,
    pub starting_equipment:HashMap<APIReference,i64>,
    pub starting_equipment_options:Choice,
    pub language_option:Choice,
    pub feature:Feature,
    pub personality:Choice,
    pub ideals:Choice,
    pub bonds:Choice,
    pub flaws:Choice,
}

impl Background{
    pub fn new() -> Self{
        Background{
            reference:APIReference::new(),
            starting_proficiencies:vec![],
            starting_equipment: HashMap::new(),
            starting_equipment_options:Choice::new(),
            language_option:Choice::new(),
            feature:Feature::new(),
            personality:Choice::new(),
            ideals:Choice::new(),
            bonds:Choice::new(),
            flaws:Choice::new(),
        }
    }
}

#[async_trait]
impl Convert for Background{
    async fn from_value(&mut self, json: Value) {
        match json.get("index"){
            Some(T) => {
                self.reference.index = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("name"){
            Some(T) => {
                self.reference.name = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("url"){
            Some(T) => {
                self.reference.url = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("starting_proficiencies"){
            Some(T) => {
                let _sp = T.as_array().unwrap();
                print!("{:?}",_sp[0]);
                for i in _sp{
                    let mut _api_ref = APIReference::new();
                    let i_object = i.as_object().unwrap();
                    _api_ref.name = i_object["name"].as_str().unwrap().to_string();
                    _api_ref.url = i_object["url"].as_str().unwrap().to_string();
                    _api_ref.index = i_object["index"].as_str().unwrap().to_string();
                    self.starting_proficiencies.push(_api_ref);
                }
            },
            None => print!("?"),
        }
        match json.get("starting_equipment"){
            Some(T) => {
                let _sp = T.as_array().unwrap();
                for i in _sp{
                    let mut _api_ref = APIReference::new();
                    let i_object = i.as_object().unwrap();
                    let equipment = i_object["equipment"].as_object().unwrap();
                    _api_ref.name = equipment["name"].as_str().unwrap().to_string();
                    _api_ref.url = equipment["url"].as_str().unwrap().to_string();
                    _api_ref.index = equipment["index"].as_str().unwrap().to_string();
                    let quantity = i_object["quantity"].as_i64().unwrap();
                    self.starting_equipment.insert(_api_ref,quantity);
                }
            },
            None => print!("?"),
        }
        match json.get("personality_traits"){
            Some(T) => {
                self.personality.parse(T).await;
            },
            None => print!("?"),
        }
        match json.get("language_options"){
            Some(T) => {
                self.language_option.parse(T).await;
            },
            None => print!("?"),
        }
        match json.get("starting_equipment_options"){
            Some(T) => {
                self.starting_equipment_options.parse(&T.as_array().unwrap()[0]).await;
            }
            None => print!("?"),
        }
        match json.get("feature"){
            Some(T) => {
                if Some(T["name"].as_str()).is_some() && Some(T["desc"].as_array()).is_some(){
                    self.feature.feature_type = T["name"].as_str().unwrap().to_string();
                    let mut desc_array = T["desc"].as_array().unwrap();
                    for i in desc_array{
                        self.feature.desc.push(i.as_str().unwrap().to_string());
                    }
                }
            }
            None => print!("?"),
        }
        match json.get("ideals"){
            Some(T) => {
                self.ideals.parse(T).await;
            }
            None => print!("?"),
        }
        match json.get("bonds"){
            Some(T) => {
                self.bonds.parse(T).await;
            }
            None => print!("?"),
        }
        match json.get("flaws"){
            Some(T) => {
                self.flaws.parse(T).await;
            }
            None => print!("?"),
        }
    }
}

pub async fn send_background_response(ctx: &Context, msg: &Message, ali_type:String) -> CommandResult{
    if ali_type != "all".to_string()
    {
        let client = Client::new();
        let res = client.get(format!("{}{}{}", API_SERVER, BACKGROUND_LINK, ali_type.to_string()))
            .send()
            .await
            .expect("fail to get to link")
            .text()
            .await
            .expect("fail to convert to json");
        let json: serde_json::Value = from_str(&res).expect("what?");
        let mut a = Background::new();
        a.from_value(json.clone()).await;

        //turn fields to string for display
        let mut feature: String = "".to_string();
        let mut starting_profi: String = "".to_string();
        let mut starting_equipments: String = "".to_string();
        let personality: String = a.personality.display().await;
        let mut starting_equipment_option: String = "".to_string();
        let ideals: String = a.ideals.display().await;
        let flaws: String = a.flaws.display().await;
        let bonds: String = a.bonds.display().await;

        for i in a.starting_proficiencies
        {
            starting_profi += &*format!("*{}*\n", i.name);
        }
        for i in a.starting_equipment
        {
            starting_equipments += &*format!("*{}*({})\n", i.0.name, i.1)
        }
        starting_equipment_option += &*format!("*{}* ({}{})", a.starting_equipment_options.from.equipment_category.name,
                                               API_SERVER,
                                               a.starting_equipment_options.from.equipment_category.url);
        for i in a.feature.desc {
            feature += &*format!("*{}*\n", i);
        }

        let mut embed = CreateEmbed::new()
            .title(format!("{}", a.reference.name))
            .field(a.feature.feature_type, feature, false)
            .field(format!("Personality traits ({} choice(s))", a.personality.choose),
                   personality, false)
            .field(format!("Flaws ({} choice(s))", a.flaws.choose), flaws, false)
            .field(format!("Bonds ({} choice(s))", a.bonds.choose), bonds, false)
            .field("Starting Proficiencies", starting_profi, false)
            .field("Starting Equipments", starting_equipments, false)
            .field(format!("Starting Equipment Option ({} choice(s))", a.starting_equipment_options.choose),
                   starting_equipment_option, false)
            .field(format!("Ideals ({} choice(s))", a.ideals.choose), ideals, false)
            .field(format!("Languages ({} choice(s))", a.language_option.choose),
                   format!("URL: {}{}", API_SERVER, a.language_option.from.resource_list_url), false);
        if a.reference.url != "" {
            embed = embed.clone().url(format!("{}{}", API_SERVER, a.reference.url).to_string());
        }
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content("test!")
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    else
    {
        let mut embed = CreateEmbed::new()
            .title("**All available Backgrounds**");
        for i in &RESOURCES_LIST["backgrounds"].results
        {
            embed = embed.clone().field(format!("{}", i.name), format!("{}", i.index), true);
        }
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(ali_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    Ok(())
}
