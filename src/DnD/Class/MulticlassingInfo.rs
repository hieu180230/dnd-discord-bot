use std::collections::HashMap;
use std::ops::Mul;
use std::string::ToString;
use reqwest::{Client};

use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use serde_json::{from_str, Value};

use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Schemas::{APIReference, Choice, ScorePrerequisite};
use crate::DnD::CharData::{Convert};

pub struct Multiclassing{
    pub prerequisites:Vec<ScorePrerequisite>,
    pub prerequisite_options: Vec<Choice>,
    pub proficiencies: Vec<APIReference>,
    pub proficiency_choices: Vec<Choice>,
}

impl Multiclassing{
    pub fn new() -> Self{
        Multiclassing{
            prerequisites:vec![],
            prerequisite_options:vec![],
            proficiencies:vec![],
            proficiency_choices:vec![],
        }
    }
}

#[async_trait]
impl Convert for Multiclassing{
    async fn from_value(&mut self, json: Value) {
        match json.get("prerequisites"){
            Some(T) => {
                let pre_array = T.as_array().unwrap();
                for pre in pre_array{
                    self.prerequisites.push(ScorePrerequisite::parse(pre));
                }
            }
            None => print!("?"),
        }
        match json.get("proficiencies"){
            Some(T) => {
                let pro_array = T.as_array().unwrap();
                for pro in pro_array{
                    self.proficiencies.push(APIReference::parse(pro));
                }
            }
            None => print!("?")
        }
        match json.get("proficiency_choices"){
            Some(T) => {
                let pro_choices = T.as_array().unwrap();
                for choice in pro_choices{
                    let mut c:Choice = Choice::new();
                    c.parse(choice).await;
                    self.proficiency_choices.push(c);
                }
            }
            None => print!("?"),
        }
        match json.get("prerequisite_options"){
            Some(T) => {
                if Some(T.as_object().unwrap()).is_some() {
                    let mut c = Choice::new();
                    c.parse(T).await;
                    self.prerequisite_options.push(c);
                }
                else if Some(T.as_array().unwrap()).is_some() {
                    let pre_choices = T.as_array().unwrap();
                    for choice in pre_choices {
                        let mut c: Choice = Choice::new();
                        c.parse(choice).await;
                        self.proficiency_choices.push(c);
                    }
                }
            }
            None => print!("?"),
        }
    }
}