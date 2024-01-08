use crate::DnD::Schemas::APIReference;
use crate::DnD::{Convert};
use crate::DnD::{API_SERVER, RESOURCES_LIST};

use std::string::ToString;
use reqwest::{Client};
use serde_json::from_str;

use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::builder::CreateEmbed;


const PROFICIENCIES_URL: &str = "/api/proficiencies/";

pub struct Proficiencies{
    pub reference:APIReference,
    pub proficiencies_type:String,
    pub classes:Vec<APIReference>,
    pub races:Vec<APIReference>,
    pub references:APIReference,
}

impl Proficiencies{
    pub fn new() -> Self{
        Proficiencies{
            reference: APIReference::new(),
            proficiencies_type: "".to_string(),
            classes: vec![],
            races: vec![],
            references: APIReference::new(),
        }
    }
}

#[async_trait]
impl Convert for Proficiencies{
    async fn from_value(&mut self, json:serde_json::Value) {
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
        match json.get("type"){
            Some(T) => {
                self.proficiencies_type = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("classes"){
            Some(T) => {
                for i in T.as_array().unwrap(){
                    self.classes.push(APIReference{index:i["index"].as_str().unwrap().to_string(),
                        name:i["name"].as_str().unwrap().to_string(),
                        url:i["url"].as_str().unwrap().to_string()})
                }
            },
            None => print!("?"),
        }
        match json.get("races"){
            Some(T) => {
                for i in T.as_array().unwrap(){
                    self.races.push(APIReference{index:i["index"].as_str().unwrap().to_string(),
                        name:i["name"].as_str().unwrap().to_string(),
                        url:i["url"].as_str().unwrap().to_string()})
                }
            },
            None => print!("?"),
        }
        match json.get("reference"){
            Some(T) => {
                let i = T.as_object().unwrap();
                self.references = APIReference{index:i["index"].as_str().unwrap().to_string(),
                    name:i["name"].as_str().unwrap().to_string(),
                    url:i["url"].as_str().unwrap().to_string()}
            }
            None => print!("?"),
        }
    }
}

pub async fn send_proficiencies_response(ctx: &Context, msg: &Message, prf_type:String) -> CommandResult{
    if prf_type != "all".to_string()
    {
        let client = Client::new();
        let res = client.get(format!("{}{}{}", API_SERVER, PROFICIENCIES_URL, prf_type.to_string()))
            .send()
            .await
            .expect("fail to get to link")
            .text()
            .await
            .expect("fail to convert to json");
        let json: serde_json::Value = from_str(&res).expect("what?");
        let mut a = Proficiencies::new();
        a.from_value(json.clone()).await;

        let mut races: String = "".to_string();
        let mut classes: String = "".to_string();
        for race in &a.races {
            races += &*format!("+ *{}*\n", race.name);
        }
        for class in &a.classes {
            classes += &*format!("+ *{}*\n", class.name);
        }

        let mut embed = CreateEmbed::new()
            .title(format!("{}", a.reference.name))
            .field("Type", format!("{}", a.proficiencies_type), false)
            .fields(vec![("Classes",
                          (|| -> String { if a.classes.len() == 0 { return "None".to_string(); } else { return classes; } })(),
                          true),
                         ("Races",
                          (|| -> String { if a.races.len() == 0 { return "None".to_string(); } else { return races; } })(),
                          true)]);
        if a.references.url != "" {
            embed = embed.clone().url(format!("{}{}", API_SERVER, a.references.url).to_string());
        }
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(prf_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    else
    {
        let mut embed = CreateEmbed::new()
            .title("**All available Proficiencies**");
        for i in &RESOURCES_LIST["proficiencies"].results
        {
            embed = embed.clone().field(format!("{}", i.name), format!("{}", i.index), true);
        }
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(prf_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    Ok(())
}
