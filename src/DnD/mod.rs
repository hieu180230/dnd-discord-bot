use crate::DnD::Schemas::APIReferenceList;
use lazy_static::lazy_static;
use serenity::all::standard::CommandResult;
use serenity::all::Message;
use serenity::async_trait;
use serenity::client::Context;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::Into;
use std::ops::Index;
use tokio::task;

pub mod CharData;
pub mod Class;
pub mod DnDCommands;
pub mod Equipment;
pub mod Mechanic;
pub mod Schemas;
pub mod SchemasUtils;

pub const API_SERVER: &str = "https://www.dnd5eapi.co";

fn initialize_resources_list() -> HashMap<String, APIReferenceList> {
    task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(APIReferenceList::load())
    })
}

lazy_static! {
    pub static ref RESOURCES_LIST: HashMap<String, APIReferenceList> =
        { initialize_resources_list() };
}

#[async_trait]
pub trait Convert {
    async fn from_value(&mut self, json: serde_json::Value);
}

#[async_trait]
pub trait SendResponse {
    async fn send_response(ctx: &Context, msg: &Message, _type: Vec<&str>) -> CommandResult;
}
