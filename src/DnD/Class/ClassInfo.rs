use std::collections::HashMap;
use std::string::ToString;
use std::time::Duration;
use reqwest::{Client};

use serenity::futures::StreamExt;
use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{ComponentInteractionDataKind, CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::builder::{CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};

use serde_json::{from_str, Value};

use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Schemas::{APIReference, Choice};
use crate::DnD::CharData::Convert;
use crate::DnD::Class::{MulticlassingInfo::*, SpellcastingInfo::*};

const CLASS_URL: &str = "/api/classes/";

pub struct ClassInfo{
    pub reference: APIReference,
    pub hit_die: i64,
    pub class_levels: String,
    pub multi_classing: Multiclassing,
    pub spellcasting: SpellCasting,
    pub spells: String,
    pub starting_equipment: HashMap<APIReference, i64>,
    pub starting_equipment_options: Vec<Choice>,
    pub proficiency_choices: Vec<Choice>,
    pub proficiencies: Vec<APIReference>,
    pub saving_throws: Vec<APIReference>,
    pub subclasses: Vec<APIReference>,
}

impl ClassInfo{
    pub fn new() -> Self{
        ClassInfo{
            reference:APIReference::new(),
            hit_die:-1,
            class_levels:"".to_string(),
            multi_classing:Multiclassing::new(),
            spellcasting:SpellCasting::new(),
            spells:"".to_string(),
            starting_equipment:HashMap::new(),
            starting_equipment_options:vec![],
            proficiency_choices:vec![],
            proficiencies:vec![],
            saving_throws:vec![],
            subclasses:vec![],
        }
    }
}

#[async_trait]
impl Convert for ClassInfo {
    async fn from_value(&mut self, json: Value) {
        match json.get("index") {
            Some(T) => {
                self.reference.index = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("name") {
            Some(T) => {
                self.reference.name = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("url") {
            Some(T) => {
                self.reference.url = T.as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        match json.get("hit_die") {
            Some(T) => {
                self.hit_die = T.as_i64().unwrap();
            }
            None => print!("?")
        }
        match json.get("class_levels") {
            Some(T) => {
                self.class_levels = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("spells") {
            Some(T) => {
                self.spells = T.as_str().unwrap().to_string();
            }
            None => print!("?"),
        }
        match json.get("multi_classing") {
            Some(T) => {
                let mut multi_classing = Multiclassing::new();
                multi_classing.from_value(T.clone()).await;
                self.multi_classing = multi_classing;
            }
            None => print!("?"),
        }
        match json.get("spellcasting") {
            Some(T) => {
                let mut spellcasting = SpellCasting::new();
                spellcasting.from_value(T.clone()).await;
                self.spellcasting = spellcasting;
            }
            None => print!("?")
        }
        match json.get("starting_equipment") {
            Some(T) => {
                let _se = T.as_array().unwrap();
                for i_object in _se {
                    let _api_ref = APIReference::parse(&i_object["equipment"]);
                    let quantity = i_object["quantity"].as_i64().unwrap();
                    self.starting_equipment.insert(_api_ref, quantity);
                }
            }
            None => print!("?"),
        }
        match json.get("starting_equipment_options"){
            Some(T) => {
                for option in T.as_array().unwrap(){
                    let mut choice = Choice::new();
                    choice.parse(option).await;
                    self.starting_equipment_options.push(choice);
                }
            }
            None => print!("?"),
        }
        match json.get("proficiency_choices"){
            Some(T) => {
                for option in T.as_array().unwrap(){
                    let mut choice = Choice::new();
                    choice.parse(option).await;
                    self.proficiency_choices.push(choice);
                }
            }
            None => print!("?"),
        }
        match json.get("proficiencies"){
            Some(T) => {
                for pro in T.as_array().unwrap(){
                    self.proficiencies.push(APIReference::parse(pro));
                }
            }
            None => print!("?"),
        }
        match json.get("saving_throws"){
            Some(T) => {
                for pro in T.as_array().unwrap(){
                    self.saving_throws.push(APIReference::parse(pro));
                }
            }
            None => print!("?"),
        }
        match json.get("subclasses"){
            Some(T) => {
                for pro in T.as_array().unwrap(){
                    self.subclasses.push(APIReference::parse(pro));
                }
            }
            None => print!("?"),
        }
    }
}

pub async fn send_class_response(ctx: &Context, msg: &Message, _type:String) -> CommandResult{
    if _type != "all".to_string()
    {
        //get the resource from API
        let client = Client::new();
        let res = client.get(format!("{}{}{}", API_SERVER, CLASS_URL, _type.to_string()))
            .send()
            .await
            .expect("fail to get to link")
            .text()
            .await
            .expect("fail to convert to json");
        let json: serde_json::Value = from_str(&res).expect("what?");
        let mut a = ClassInfo::new();
        a.from_value(json.clone()).await;

        //turn fields into String for display
        let mut title: String = format!("{}",a.reference.name);
        if a.hit_die != -1 { title += &*format!(" ({} hit(s) to death)", a.hit_die); }
        //if a.class_levels != "" { title += &*format!(" (Level resource: {}{})", API_SERVER, a.class_levels); }
        let mut proficiency_choice:String = "".to_string();
        for pro in a.proficiency_choices{
            let pro_display = pro.display().await;
            proficiency_choice += &*format!("{}\n",pro_display)
        }
        let proficiencies = APIReference::display(a.proficiencies);

        //Create the embed for the main message
        let mut embed = CreateEmbed::new()
            .title(title);
        if a.reference.url != "" {
            embed = embed.clone().url(format!("{}{}", API_SERVER, a.reference.url).to_string());
        }
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(_type)
            .embed(embed)
            .select_menu(CreateSelectMenu::new("Component select", CreateSelectMenuKind::String {
                options:vec![
                    CreateSelectMenuOption::new("Proficiency choices", "Proficiency choices"),
                    CreateSelectMenuOption::new("Proficiencies", "Proficiencies"),
                ]
            }).placeholder("No components selected")
            );

        //Send the message and wait for interactions
        let m = msg.channel_id.send_message(&ctx.http, builder).await.unwrap();

        let mut _interaction = match m
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60))
            .await
        {
            Some(x) => x,
            None => {
                return Ok(())
            },
        };

        //get what component to display and its value
        let option = match &_interaction.data.kind {
            ComponentInteractionDataKind::StringSelect {
                values,
            } => &values[0],
            _ => panic!("unexpected interaction data kind"),
        };
        let option_value = match option.as_str(){
            "Proficiency choices" => proficiency_choice,
            "Proficiencies" => proficiencies,
            _ => {"".to_string()},
        };

        _interaction
            .create_response(
                &ctx,
                // This time we dont edit the message but reply to it
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::default()
                        // Make the message hidden for other users by setting `ephemeral(true)`.
                        .ephemeral(true)
                        .content(format!("Proficiency choices"))
                        .embed(CreateEmbed::new()
                                   .title(format!("{} for {}",option, a.reference.name))
                                   .field("", format!("{}",option_value), false),
                        ),
                ))
            .await
            .unwrap();
        m.delete(&ctx).await.unwrap();
    }
    else
    {
        let mut embed = CreateEmbed::new()
            .title("**All available Classes**");
        for i in &RESOURCES_LIST["classes"].results
        {
            embed = embed.clone().field(format!("{}", i.name), format!("{}", i.index), true);
        }
        embed = embed.clone().timestamp(Timestamp::now());
        let builder = CreateMessage::new()
            .content(_type)
            .embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error {:?}", why);
        }
    }
    Ok(())
}