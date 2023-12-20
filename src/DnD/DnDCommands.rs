use serenity::all::standard::macros;
use serenity::framework::standard::*;
use crate::DnD::CharData::AbilityScore::*;
use crate::DnD::CharData::Alignments::*;
use crate::DnD::CharData::Background::*;
use crate::DnD::CharData::Language::*;
use crate::DnD::CharData::proficiencies::*;
use crate::DnD::CharData::Skills::*;

use serenity::prelude::*;
use serenity::all::{CreateMessage, Message, Timestamp};
use crate::DnD::RESOURCES_LIST;


pub async fn str_from_vec(a:Vec<String>) -> String{
    let mut res:String = "".to_string();
    for i in &a{
        res += i;
        res += "\n";
    }
    return res;
}

#[macros::group]
#[prefix(DnD)]
#[only_in(guilds)]
#[commands(look_up_abi, look_up_ali, look_up_bg, look_up_language, look_up_proficiency, look_up_skill)]
struct DnD;

#[macros::command]
#[aliases(ability)]
async fn look_up_abi(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_abi_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["ability-scores"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_abi_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(alignment)]
async fn look_up_ali(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_alignment_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["alignments"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_alignment_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(background)]
async fn look_up_bg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_background_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["backgrounds"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_background_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(language)]
async fn look_up_language(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_language_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["languages"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_language_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(proficiency)]
async fn look_up_proficiency(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_proficiencies_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["proficiencies"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_proficiencies_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(skill)]
async fn look_up_skill(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == "all".to_string(){
        send_skill_response(ctx, msg, "all".to_string()).await.expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["skills"].results{
        if alias.current().unwrap_or_default().to_lowercase() == i.index.to_string() {
            send_skill_response(ctx, msg, i.index.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}