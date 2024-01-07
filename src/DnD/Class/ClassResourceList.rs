use std::collections::HashMap;
use std::string::ToString;
use std::time::Duration;
use reqwest::{Client};

use serenity::futures::StreamExt;
use serenity::async_trait;
use serenity::framework::standard::*;
use serenity::all::{ComponentInteractionDataKind, CreateMessage, Message, Timestamp};
use serenity::prelude::*;
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};

use serde_json::{from_str, Value};

use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Schemas::{APIReference, APIReferenceList, Choice};
use crate::DnD::CharData::Convert;

#[async_trait]
impl Convert for APIReferenceList{
    async fn from_value(&mut self, json: Value) {
        match json.get("count"){
            Some(T) => self.count = T.as_i64().unwrap(),
            None => print!("?"),
        }
        match json.get("results"){
            Some(T) => {
                for reference in T.as_array().unwrap()
                {
                    self.results.push(
                        APIReference {
                            index: reference["index"].as_str().unwrap().to_string(),
                            name: reference["name"].as_str().unwrap().to_string(),
                            url: reference["url"].as_str().unwrap().to_string()})
                }
            }
            None => print!("?"),
        }
    }
}