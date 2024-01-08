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
use serde_json::de::Read;

use crate::DnD::{API_SERVER, RESOURCES_LIST};
use crate::DnD::Schemas::{APIReference, APIReferenceList, Choice};
use crate::DnD::Convert;

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

pub async fn send_class_resource_response(ctx: &Context, msg: &Message, _type:Vec<String>) -> CommandResult{
    let API_Endpoint = format!("/api/classes/{}/{}", _type[0], _type[1]);
    let client = Client::new();
    let res = client.get(format!("{}{}", API_SERVER, API_Endpoint))
        .send()
        .await
        .expect("fail to get to link")
        .text()
        .await
        .expect("fail to convert to json");
    let json: serde_json::Value = from_str(&res).expect("what?");
    let mut a = APIReferenceList::new();
    a.from_value(json.clone()).await;

    let (builder, display_data) = a.display(_type.clone()).await;

    let m = msg.channel_id.send_message(&ctx.http, builder).await.unwrap();

    let mut interaction = m.await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(200)).stream();


    let mut count = 0;
    while let Some(user_interaction) = interaction.next().await{
        let option = &user_interaction.data.custom_id;
        let mut embed = CreateEmbed::new();
        match option.as_str(){
            "first" => {
                println!("first");
                count = 0;
                for i in 0..display_data[count].len(){
                    embed = embed.field(&*display_data[count][i].name, &*display_data[count][i].index, true);
                }
            }
            "prev" => {
                println!("prev");
                if count > 0 {count -= 1;}
                for i in 0..display_data[count].len(){
                    embed = embed.field(&*display_data[count][i].name, &*display_data[count][i].index, true);
                }
            }
            "next" => {
                println!("next");
                if count < display_data.len() - 1 {count += 1;}
                for i in 0..display_data[count].len(){
                    embed = embed.field(&*display_data[count][i].name, &*display_data[count][i].index, true);
                }
            }
            "last" => {
                println!("last");
                count = display_data.len() - 1;
                for i in 0..display_data[count].len(){
                    embed = embed.field(&*display_data[count][i].name, &*display_data[count][i].index, true);
                }
            }
            _ => {}
        }
        embed = embed.title(format!("{} of {} ({})", _type[1], _type[0], count + 1));
        user_interaction
            .create_response(&ctx,
                             // This time we dont edit the message but reply to it
                             CreateInteractionResponse::UpdateMessage(
                                 CreateInteractionResponseMessage::default()
                                     // Make the message hidden for other users by setting `ephemeral(true)`.
                                     .ephemeral(false)
                                     .content(format!("{} of {}", _type[1], _type[0]))
                                     .embed(embed.clone()),
                             )
            ).await.unwrap();
    }
    Ok(())
}