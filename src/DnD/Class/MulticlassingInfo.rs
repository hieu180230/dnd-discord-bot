use std::string::ToString;

use serenity::async_trait;
use serenity::prelude::*;

use serde_json::Value;

use crate::DnD::Convert;
use crate::DnD::Schemas::{APIReference, Choice, ScorePrerequisite};

pub struct Multiclassing {
    pub prerequisites: Vec<ScorePrerequisite>,
    pub prerequisite_options: Vec<Choice>,
    pub proficiencies: Vec<APIReference>,
    pub proficiency_choices: Vec<Choice>,
}

impl Multiclassing {
    pub fn new() -> Self {
        Multiclassing {
            prerequisites: vec![],
            prerequisite_options: vec![],
            proficiencies: vec![],
            proficiency_choices: vec![],
        }
    }
}
impl Default for Multiclassing {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Convert for Multiclassing {
    async fn from_value(&mut self, json: Value) {
        match json.get("prerequisites") {
            Some(T) => {
                let pre_array = T.as_array().unwrap();
                for pre in pre_array {
                    self.prerequisites.push(ScorePrerequisite::parse(pre));
                }
            }
            None => print!("?"),
        }
        match json.get("proficiencies") {
            Some(T) => {
                let pro_array = T.as_array().unwrap();
                for pro in pro_array {
                    self.proficiencies.push(APIReference::parse(pro));
                }
            }
            None => print!("?"),
        }
        match json.get("proficiency_choices") {
            Some(T) => {
                let pro_choices = T.as_array().unwrap();
                for choice in pro_choices {
                    let mut c: Choice = Choice::new();
                    c.parse(choice).await;
                    self.proficiency_choices.push(c);
                }
            }
            None => print!("?"),
        }
        match json.get("prerequisite_options") {
            Some(T) => {
                if Some(T.as_object().unwrap()).is_some() {
                    let mut c = Choice::new();
                    c.parse(T).await;
                    self.prerequisite_options.push(c);
                } else if Some(T.as_array().unwrap()).is_some() {
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

impl Multiclassing {
    pub async fn display(&self) -> String {
        let mut res = "".to_string();
        //prerequisites
        if !self.prerequisites.is_empty() {
            res += "**Prerequisites**\n";
            for pre in &self.prerequisites {
                res += &*format!(
                    "- *{} (minimum score: {})*\n",
                    pre.ability_score.name, pre.minimum_score
                );
            }
        }
        //prerequisite choices
        if !self.prerequisite_options.is_empty() {
            res += "**Prerequisite choices**\n";
            for choice in &self.prerequisite_options {
                res += &*choice.display(1).await;
            }
        }
        //proficiencies
        if !self.proficiencies.is_empty() {
            res += "**Proficiencies**\n";
            for pro in &self.proficiencies {
                res += &*format!("- *{}*\n", pro.name)
            }
        }
        //proficiency choices
        if !self.proficiency_choices.is_empty() {
            res += "**Proficiency choices**\n";
            for choice in &self.proficiency_choices {
                res += &*format!("* {}", choice.display(1).await);
            }
        }

        res
    }
}
