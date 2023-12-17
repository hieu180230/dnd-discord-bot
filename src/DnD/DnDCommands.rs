use serenity::all::standard::macros;
use serenity::framework::standard::*;
use crate::DnD::CharData::AbilityScore::*;
use crate::DnD::CharData::Alignments::*;
use crate::DnD::CharData::Background::*;
use crate::DnD::CharData::Language::*;

use serenity::prelude::*;
use serenity::all::{CreateMessage, Message, Timestamp};


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
#[commands(look_up_abi, look_up_ali, look_up_bg, look_up_language)]
struct DnD;

#[macros::command]
#[aliases(ability_lookup)]
async fn look_up_abi(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    for i in ABILITY_SCORE_ALIASES {
        if alias.current().unwrap_or_default().to_lowercase() == i.alias ||
            alias.current().unwrap_or_default().to_lowercase() == i.link {
            send_abi_response(ctx, msg, i.link.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(alignment_lookup)]
async fn look_up_ali(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    for i in ALIGNMENTS{
        if alias.current().unwrap_or_default().to_lowercase() == i.to_string(){
            send_alignment_response(ctx, msg, i.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(background_lookup)]
async fn look_up_bg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    for i in BACKGROUND{
        if alias.current().unwrap_or_default().to_lowercase() == i.to_string(){
            send_background_response(ctx, msg, i.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}

#[macros::command]
#[aliases(language_lookup)]
async fn look_up_language(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    for i in LANGUAGE{
        if alias.current().unwrap_or_default().to_lowercase() == i.to_string(){
            send_language_response(ctx, msg, i.to_string()).await.expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(ctx, format!("Unknown alias: {:?}", alias.current().unwrap_or_default())).await?;
    Ok(())
}
