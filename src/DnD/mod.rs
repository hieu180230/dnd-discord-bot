use std::clone::Clone;
use std::collections::HashMap;
use std::convert::Into;
use std::ops::Index;
use lazy_static::lazy_static;
use tokio::task;
use crate::DnD::Schemas::APIReferenceList;


pub mod Schemas;
pub mod CharData;
pub mod DnDCommands;
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
    pub static ref RESOURCES_LIST: HashMap<String, APIReferenceList> = {
        initialize_resources_list()
    };
}