use crate::DnD::CharData::AbilityScore::*;
use crate::DnD::CharData::Alignments::*;
use crate::DnD::CharData::Background::*;
use crate::DnD::CharData::Language::*;
use crate::DnD::CharData::Proficiencies::*;
use crate::DnD::CharData::Skills::*;
use crate::DnD::Class::ClassInfo::*;
use crate::DnD::Class::ClassLevel::*;
use crate::DnD::Mechanic::GameMechanic::*;
use crate::DnD::Schemas::APIReferenceList;
use crate::DnD::SendResponse;
use crate::DnD::RESOURCES_LIST;

use serenity::all::standard::macros;
use serenity::all::Message;
use serenity::framework::standard::*;
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
    look_up_class,
    look_up_mechanic
)]
struct DnD;

#[macros::command]
#[aliases(ability)]
pub async fn look_up_abi(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let alias = args.clone();
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        AbilityScore::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["ability-scores"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            AbilityScore::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        Alignment::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["alignments"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            Alignment::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        Background::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["backgrounds"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            Background::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        Language::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["languages"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            Language::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        Proficiencies::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["proficiencies"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            Proficiencies::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        Skill::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["skills"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            Skill::send_response(ctx, msg, vec![i.index.as_str()])
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
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        ClassInfo::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    for i in &RESOURCES_LIST["classes"].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            if alias.advance().current().is_none() {
                ClassInfo::send_response(ctx, msg, vec![i.index.as_str()])
                    .await
                    .expect("TODO: panic message");
                return Ok(());
            }
            match alias.current().unwrap_or_default().to_lowercase().as_str() {
                "subclasses" => {
                    APIReferenceList::send_response(ctx, msg, vec![i.index.as_str(), "subclass"])
                        .await
                        .expect("TODO: panic message")
                }
                "spells" => {
                    APIReferenceList::send_response(ctx, msg, vec![i.index.as_str(), "spells"])
                        .await
                        .expect("TODO: panic message")
                }
                "features" => {
                    APIReferenceList::send_response(ctx, msg, vec![i.index.as_str(), "features"])
                        .await
                        .expect("TODO: panic message")
                }
                "proficiencies" => APIReferenceList::send_response(
                    ctx,
                    msg,
                    vec![i.index.as_str(), "proficiencies"],
                )
                .await
                .expect("TODO: panic message"),
                "levels" => {
                    if alias.advance().current().is_none() {
                        ClassLevel::send_response(ctx, msg, vec![i.index.as_str(), "", "", ""])
                            .await
                            .expect("TODO: panic message");
                    } else {
                        if alias
                            .current()
                            .unwrap_or_default()
                            .to_string()
                            .parse::<i64>()
                            .is_ok()
                        {
                            let level = format!("/{}", alias.current().unwrap_or_default());
                            let mut option = "".to_string();
                            if alias.advance().current().is_some() {
                                option = format!(
                                    "/{}",
                                    alias.current().unwrap_or_default().to_lowercase()
                                );
                            }
                            ClassLevel::send_response(
                                ctx,
                                msg,
                                vec![i.index.as_str(), "", &level, &option],
                            )
                            .await
                            .expect("TODO: panic message");
                            return Ok(());
                        }
                        let subclass = format!("?subclass={}", alias.current().unwrap_or_default());
                        ClassLevel::send_response(
                            ctx,
                            msg,
                            vec![i.index.as_str(), &subclass, "", ""],
                        )
                        .await
                        .expect("TODO: panic message");
                        return Ok(());
                    }
                }
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

#[macros::command]
#[aliases(mechanic)]
async fn look_up_mechanic(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut alias = args.clone();
    //show all mechanics
    if alias.current().unwrap_or_default().to_lowercase() == *"all" || alias.current().is_none() {
        GameMechanic::send_response(ctx, msg, vec!["all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    //get the mechanic index
    let resource_endpoint = match alias.current().unwrap_or_default().to_lowercase().as_str() {
        "conditions" => "conditions",
        "damagetypes" => "damage-types",
        "magicschools" => "magic-schools",
        _ => "none",
    };
    alias.advance();
    //show all mechanics of a type
    if alias.current().is_none() || alias.current().unwrap_or_default().to_lowercase() == *"all" {
        GameMechanic::send_response(ctx, msg, vec![resource_endpoint, "all"])
            .await
            .expect("TODO: panic message");
        return Ok(());
    }
    //show a specific mechanic of a type
    for i in &RESOURCES_LIST[resource_endpoint].results {
        if alias.current().unwrap_or_default().to_lowercase() == i.index {
            GameMechanic::send_response(ctx, msg, vec![resource_endpoint, i.index.as_str()])
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
