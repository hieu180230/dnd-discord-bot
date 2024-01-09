use crate::DnD::CharData::proficiencies::*;
use crate::DnD::CharData::AbilityScore::*;
use crate::DnD::CharData::Alignments::*;
use crate::DnD::CharData::Background::*;
use crate::DnD::CharData::Language::*;
use crate::DnD::CharData::Skills::*;
use serenity::all::standard::macros;
use serenity::framework::standard::*;

use crate::DnD::Class::ClassInfo::*;

use crate::DnD::Class::ClassResourceList::send_class_resource_response;
use crate::DnD::RESOURCES_LIST;
use serenity::all::Message;
use serenity::prelude::*;

pub async fn str_from_vec(a: Vec<String>) -> String {
    let mut res: String = "".to_string();
    for i in &a {
        res += i;
        res += "\n";
    }

    res
}

#[macros::group]
#[prefix(DnD)]
#[only_in(guilds)]
#[commands(
    look_up_abi,
    look_up_ali,
    look_up_bg,
    look_up_language,
    look_up_proficiency,
    look_up_skill,
    look_up_class
)]
struct DnD;

#[macros::command]
#[aliases(ability)]
async fn look_up_abi(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_abi_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["ability-scores"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_abi_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(alignment)]
async fn look_up_ali(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_alignment_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["alignments"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_alignment_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(background)]
async fn look_up_bg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_background_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["backgrounds"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_background_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(language)]
async fn look_up_language(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_language_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["languages"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_language_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(proficiency)]
async fn look_up_proficiency(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_proficiencies_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["proficiencies"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_proficiencies_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(skill)]
async fn look_up_skill(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_skill_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["skills"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            send_skill_response(ctx, msg, i.index.to_string())
                .await
                .expect("TODO: panic message");
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}

#[macros::command]
#[aliases(class)]
async fn look_up_class(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" {
        send_class_response(ctx, msg, "all".to_string())
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["classes"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            if alias.advance().current().is_none() {
                send_class_response(ctx, msg, i.index.to_string())
                    .await
                    .expect("TODO: panic message");
                return Ok(());
            }
            match alias.current().unwrap_or_default().to_lowercase().as_str() {
                "subclasses" => send_class_resource_response(
                    ctx,
                    msg,
                    vec![i.index.to_string(), "subclass".to_string()],
                )
                .await
                .expect("TODO: panic message"),
                "spells" => send_class_resource_response(
                    ctx,
                    msg,
                    vec![i.index.to_string(), "spells".to_string()],
                )
                .await
                .expect("TODO: panic message"),
                "features" => send_class_resource_response(
                    ctx,
                    msg,
                    vec![i.index.to_string(), "features".to_string()],
                )
                .await
                .expect("TODO: panic message"),
                "proficiencies" => send_class_resource_response(
                    ctx,
                    msg,
                    vec![i.index.to_string(), "proficiencies".to_string()],
                )
                .await
                .expect("TODO: panic message"),
                _ => {}
            }
            return Ok(());
        }
    }
    msg.reply(
        ctx,
        format!("Unknown alias: {:?}", alias.current().unwrap_or_default()),
    )
    .await?;
    Ok(())
}
