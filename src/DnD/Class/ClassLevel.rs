use reqwest::Client;
use std::collections::HashMap;
use std::hash::Hash;
use std::string::ToString;
use std::time::Duration;

use serenity::all::{
    ComponentInteractionDataKind, CreateButton, CreateMessage, Message, Timestamp,
};
use serenity::async_trait;
use serenity::builder::{
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use serenity::framework::standard::*;
use serenity::futures::StreamExt;
use serenity::prelude::*;

use serde_json::de::Read;
use serde_json::{from_str, Value};

use crate::DnD::Class::ClassInfo::ClassInfo;
use crate::DnD::Class::ClassSpecific::SPFactory::{ClassType, SPConvert};
use crate::DnD::Class::ClassSpecific::*;
use crate::DnD::Schemas::{APIReference, APIReferenceList};
use crate::DnD::API_SERVER;
use crate::DnD::{Convert, SendResponse};

//This is the main struct for class level
pub struct ClassLevel {
    pub index: String,
    pub url: String,
    pub level: i32,
    pub ability_score_bonus: i32,
    pub prof_bonus: i32,
    pub feature: Vec<APIReference>,
    pub spellcasting: HashMap<String, i64>,
    pub class_specific: Box<(dyn SPConvert + Send)>,
    pub class: APIReference,
    subclass_check: bool,
    pub subclass: APIReference,
}
impl ClassLevel {
    fn new(class: &SPFactory::ClassType) -> Self {
        ClassLevel {
            index: String::new(),
            url: String::new(),
            level: -1,
            ability_score_bonus: -1,
            prof_bonus: -1,
            feature: vec![],
            spellcasting: HashMap::new(),
            class_specific: SPFactory::SPFactory::new(class),
            class: APIReference::new(),
            subclass_check: false,
            subclass: APIReference::new(),
        }
    }
}

#[async_trait]
impl Convert for ClassLevel {
    async fn from_value(&mut self, json: Value) {
        match json.get("level") {
            Some(T) => {
                self.level = T.as_i64().unwrap() as i32;
            }
            None => {
                print!("No level found")
            }
        }
        match json.get("ability_score_bonuses") {
            Some(T) => {
                self.ability_score_bonus = T.as_i64().unwrap() as i32;
            }
            None => {
                print!("No ability_score_bonus found")
            }
        }
        match json.get("prof_bonus") {
            Some(T) => {
                self.prof_bonus = T.as_i64().unwrap() as i32;
            }
            None => {
                print!("No prof_bonus found")
            }
        }
        match json.get("features") {
            Some(T) => {
                for feature in T.as_array().unwrap() {
                    let mut temp = APIReference::new();
                    temp.from_value(feature.clone()).await;
                    self.feature.push(temp);
                }
            }
            None => {
                print!("No feature_choices found")
            }
        }
        match json.get("spellcasting") {
            Some(T) => {
                for spellcasting in T.as_object().unwrap() {
                    self.spellcasting
                        .insert(spellcasting.0.to_string(), spellcasting.1.as_i64().unwrap());
                }
            }
            None => {
                print!("No spellcasting found")
            }
        }
        match json.get("class_specific") {
            Some(T) => {
                self.class_specific.from_value(T.clone()).await;
            }
            None => {
                print!("No class_specific found")
            }
        }
        match json.get("index") {
            Some(T) => {
                self.index = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No index found")
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
        match json.get("class") {
            Some(T) => {
                self.class.from_value(T.clone()).await;
            }
            None => {
                print!("No class found")
            }
        }
        match json.get("subclass") {
            Some(T) => {
                self.subclass_check = true;
                self.subclass.from_value(T.clone()).await;
            }
            None => {
                print!("No subclass found")
            }
        }
    }
}

fn paginate_class_level(class_levels: &[ClassLevel], page: i32, title: String) -> CreateEmbed {
    // turn attributes into string for display

    let mut feature = "".to_string();
    let class_specific = class_levels[page as usize].class_specific.display();
    for i in &class_levels[page as usize].feature {
        feature += &*format!("- *{}*\n", i.name);
    }
    if feature.is_empty() {
        feature = "None".to_string();
    }

    //embed information
    let mut embed = CreateEmbed::new()
        .title(title)
        .description(format!(
            "Level {}\nAbility Score Bonus {}\nProficiency Bonus {}",
            class_levels[page as usize].level,
            class_levels[page as usize].ability_score_bonus,
            class_levels[page as usize].prof_bonus
        ))
        .field("Feature", feature, false)
        .field("Class Specific", class_specific, false);
    if class_levels[page as usize].subclass_check {
        embed = embed.field(
            "Subclass",
            &class_levels[page as usize].subclass.name,
            false,
        );
    }
    // Add a timestamp for the current time
    // This also accepts a rfc3339 Timestamp
    embed = embed.clone().timestamp(Timestamp::now());

    embed
}

#[async_trait]
impl SendResponse for ClassLevel {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult {
        let client = Client::new();
        let res = client
            .get(format!(
                "{}/api/classes/{}/levels{}{}{}",
                API_SERVER, _type[0], _type[1], _type[2], _type[3]
            ))
            .send()
            .await
            .expect("fail to get to link")
            .text()
            .await
            .expect("fail to convert to json");
        let json: serde_json::Value = from_str(&res).expect("what?");
        let mut class_type = SPFactory::ClassType::Barbarian;
        match _type[0] {
            "bard" => {
                class_type = SPFactory::ClassType::Bard;
            }
            "cleric" => {
                class_type = SPFactory::ClassType::Cleric;
            }
            "druid" => {
                class_type = SPFactory::ClassType::Druid;
            }
            "fighter" => {
                class_type = SPFactory::ClassType::Fighter;
            }
            "monk" => {
                class_type = SPFactory::ClassType::Monk;
            }
            "paladin" => {
                class_type = SPFactory::ClassType::Paladin;
            }
            "ranger" => {
                class_type = SPFactory::ClassType::Ranger;
            }
            "rogue" => {
                class_type = SPFactory::ClassType::Rogue;
            }
            "sorcerer" => {
                class_type = SPFactory::ClassType::Sorcerer;
            }
            "warlock" => {
                class_type = SPFactory::ClassType::Warlock;
            }
            "wizard" => {
                class_type = SPFactory::ClassType::Wizard;
            }
            _ => {}
        }

        let mut specific_title = "".to_string();
        let mut page: i32 = 0;
        let mut class_levels: Vec<ClassLevel> = Vec::new();
        if _type[2].is_empty() {
            for class_level in json.as_array().unwrap() {
                let mut temp = ClassLevel::new(&class_type);
                temp.from_value(class_level.clone()).await;
                class_levels.push(temp);
            }
            specific_title = format!(
                "All level resources for {}",
                class_levels[page as usize].class.name
            );
        } else {
            let mut temp = ClassLevel::new(&class_type);
            temp.from_value(json).await;
            class_levels.push(temp);
            specific_title = format!("Level {}", _type[2]);
            match _type[3] {
                "features" => {
                    specific_title += &*format!(
                        " Features resources for {}",
                        class_levels[page as usize].class.name
                    );
                }
                "spells" => {
                    specific_title += &*format!(
                        " Spells resources for {}",
                        class_levels[page as usize].class.name
                    );
                }
                "" => {
                    specific_title +=
                        &*format!(" resources for {}", class_levels[page as usize].class.name);
                }
                _ => {
                    specific_title +=
                        &*format!(" resources for {}", class_levels[page as usize].class.name);
                }
            }
        }

        let mut embed = paginate_class_level(&class_levels, page, specific_title.clone());
        let mut message = CreateMessage::new()
            .content(specific_title.clone())
            .embed(embed.clone());
        if !_type[2].is_empty() || !_type[3].is_empty() {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error {:?}", why);
            }
            return Ok(());
        }
        message = message
            .button(CreateButton::new("first").label("<<"))
            .button(CreateButton::new("prev").label("<"))
            .button(CreateButton::new("next").label(">"))
            .button(CreateButton::new("last").label(">>"));
        let m = msg
            .channel_id
            .send_message(&ctx.http, message)
            .await
            .unwrap();

        let mut interaction = m.await_component_interaction(&ctx.shard).stream();
        while let Some(user_interaction) = interaction.next().await {
            let option = &user_interaction.data.custom_id;
            match option.as_str() {
                "first" => {
                    page = 0;
                    embed = paginate_class_level(&class_levels, page, specific_title.clone());
                }
                "prev" => {
                    page = page.saturating_sub(1);
                    embed = paginate_class_level(&class_levels, page, specific_title.clone());
                }
                "next" => {
                    if page < ((class_levels.len() - 1) as i32) {
                        page += 1;
                    }
                    embed = paginate_class_level(&class_levels, page, specific_title.clone());
                }
                "last" => {
                    page = (class_levels.len() - 1) as i32;
                    embed = paginate_class_level(&class_levels, page, specific_title.clone());
                }
                _ => {}
            }

            user_interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::default()
                            .ephemeral(false)
                            .embed(embed.clone()),
                    ),
                )
                .await
                .unwrap();
        }

        Ok(())
    }
}
