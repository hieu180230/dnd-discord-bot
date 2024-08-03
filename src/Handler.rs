use crate::{Cat, Manager::Manager, DnD, HELP_MESSAGE};
use std::error;
use std::marker::Send;
use std::sync::Arc;

use serenity::all::CreateEmbed;
use std::time::Duration;

use serenity::async_trait;
use serenity::builder::{
    CreateButton, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};

use serenity::framework::standard::{macros, Args, CommandResult};
use serenity::futures::StreamExt;
use serenity::gateway::ShardManager;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

//necessary struct

pub struct Handler;
pub struct ShardManagerContainer; //this is to contain shard managers
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

//command building start here


fn sound_button(name: &str, emoji: ReactionType) -> CreateButton {
    // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
    // anything that can be converted to it. For a list of that, search Trait Implementations in
    // the docs for From<...>.
    CreateButton::new(name).emoji(emoji)
}
#[async_trait]
impl EventHandler for Handler {
    //Tis is an example of message with choice
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content != "animal" {
            return;
        }

        // Ask the user for its favorite animal
        let m = msg
            .channel_id
            .send_message(
                &ctx,
                CreateMessage::new()
                    .content("Please select your favorite animal")
                    .select_menu(
                        CreateSelectMenu::new(
                            "animal_select",
                            CreateSelectMenuKind::String {
                                options: vec![
                                    CreateSelectMenuOption::new("🐈 meow", "Cat"),
                                    CreateSelectMenuOption::new("🐕 woof", "Dog"),
                                    CreateSelectMenuOption::new("🐎 neigh", "Horse"),
                                    CreateSelectMenuOption::new("🦙 hoooooooonk", "Alpaca"),
                                    CreateSelectMenuOption::new("🦀 crab rave", "Ferris"),
                                ],
                            },
                        )
                        .custom_id("animal_select")
                        .placeholder("No animal selected"),
                    ),
            )
            .await
            .unwrap();

        // Wait for the user to make a selection
        // This uses a collector to wait for an incoming event without needing to listen for it
        // manually in the EventHandler.
        let interaction = match m
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60 * 3))
            .await
        {
            Some(x) => x,
            None => {
                m.reply(&ctx, "Timed out").await.unwrap();
                return;
            }
        };

        // data.values contains the selected value from each select menus. We only have one menu,
        // so we retrieve the first
        let animal = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => &values[0],
            _ => panic!("unexpected interaction data kind"),
        };

        // Acknowledge the interaction and edit the message
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default()
                        .content(format!("You chose: **{animal}**\nNow choose a sound!"))
                        .button(sound_button("meow", "🐈".parse().unwrap()))
                        .button(sound_button("woof", "🐕".parse().unwrap()))
                        .button(sound_button("neigh", "🐎".parse().unwrap()))
                        .button(sound_button("hoooooooonk", "🦙".parse().unwrap()))
                        .button(sound_button(
                            "crab rave",
                            // Custom emojis in Discord are represented with
                            // `<:EMOJI_NAME:EMOJI_ID>`. You can see this by posting an emoji in
                            // your server and putting a backslash before the emoji.
                            //
                            // Because ReactionType implements FromStr, we can use .parse() to
                            // convert the textual emoji representation to ReactionType
                            "<:ferris:381919740114763787>".parse().unwrap(),
                        )),
                ),
            )
            .await
            .unwrap();

        // Wait for multiple interactions
        let mut interaction_stream = m
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60 * 3))
            .stream();

        while let Some(interaction) = interaction_stream.next().await {
            let sound = &interaction.data.custom_id;
            // Acknowledge the interaction and send a reply
            interaction
                .create_response(
                    &ctx,
                    // This time we dont edit the message but reply to it
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::default()
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            .ephemeral(true)
                            .content(format!("The **{animal}** says __{sound}__")),
                    ),
                )
                .await
                .unwrap();
        }

        // Delete the orig message or there will be dangling components (components that still
        // exist, but no collector is running so any user who presses them sees an error)
        m.delete(&ctx).await.unwrap()
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {:?}", command.data.name);

            //add slash interaction here
            let content = match command.data.name.as_str() {
                "cat" => Some(Cat::run(&ctx, &command.data.options(), &command).await),
                "manager" => Some(Manager::run(&ctx, &command.data.options(), &command, command.guild_id.unwrap()).await),
                _ => Some(
                    CreateEmbed::new()
                        .title("not implemented :(".to_string())
                        .description("???"),
                ),
            };
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().embed(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut commands: Vec<CreateCommand> = Vec::new();
        //register slash command here before the bot is ready to operate
        commands.push(Cat::register());
        commands.push(Manager::register());
        Command::set_global_commands(&ctx.http, commands)
            .await
            .unwrap();
        for i in Command::get_global_commands(&ctx.http).await.unwrap() {
            println!("{:?}", i.name);
        }
        //println!("{}\n", init_message);
        println!("{} is ready!", ready.user.name);
    }
}
