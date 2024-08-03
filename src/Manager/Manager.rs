use std::error;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Message, MessageBuilder, ResolvedOption, ResolvedValue};
use serenity::all::standard::Args;
use serenity::async_trait;
use serenity::framework::standard::CommandResult;
use serenity::model::{user, guild, id};
use crate::Handler::ShardManagerContainer;
use crate::HELP_MESSAGE;

pub struct Manager
{

}

pub struct Guild;


impl Manager
{
    pub fn new() -> Self
    {
        Manager{}
    }
    pub async fn getUsers(ctx: &Context, guild_id: id::GuildId) -> CreateEmbed{
        let members = guild_id.members(&ctx.http, None, None);
        let mut reply_msg = CreateEmbed::new().title("Member in this guild");
        match members.await{
            Ok(T) => {
                for member in T{
                    reply_msg = reply_msg.field(&member.user.name, &member.user.created_at().to_string(), false);
                }

            }
            Err(E) => {
                reply_msg = reply_msg.description(format!("Err: {}\n", E));
            }
        }
        reply_msg
    }
    pub async fn some_long_command(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
        msg.channel_id
            .say(&ctx.http, &format!("Arguments: {:?}", args.rest()))
            .await?;

        Ok(())
    }
    pub async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
        let data = ctx.data.read().await;

        let shardmanager = match data.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => {
                msg.reply(ctx, "problem getting shard manager")
                    .await
                    .expect("problem sending error message");
                return Ok(());
            }
        };

        let runners = shardmanager.runners.lock().await;

        let runner = match runners.get(&ctx.shard_id) {
            Some(runner) => runner,
            None => {
                msg.reply(&ctx, "No shard found")
                    .await
                    .expect("problem sending error message");
                return Ok(());
            }
        };

        msg.reply(&ctx, &format!("Latency: {:?}", runner.latency.unwrap()))
            .await
            .expect("fail to send latency");
        Ok(())
    }
    pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
        let channel = match msg.channel_id.to_channel(&ctx).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {:?}", why);
                return Err(Box::new(why) as Box<dyn error::Error + Send + Sync>);
            }
        };
        let content = MessageBuilder::new()
            .push("User ")
            .push_bold_safe(&msg.author.name) //this make the text italic or bold, ...
            .push(" ping the channel ")
            .mention(&channel)
            .build();
        let resp = CreateMessage::new().content(content).select_menu(
            CreateSelectMenu::new(
                "custom_id",
                CreateSelectMenuKind::String {
                    options: vec![
                        CreateSelectMenuOption::new("label1", "value1"),
                        CreateSelectMenuOption::new("label2", "value2"),
                    ],
                },
            )
                .placeholder("holder"),
        );
        if let Err(why) = msg.channel_id.send_message(&ctx.http, resp).await {
            println!("Error sending ping msg: {:?}", why);
        }
        return Ok(());
    }
    pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;
        Ok(())
    }
}

pub async fn run(
    ctx: &Context,
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
    guild_id: id::GuildId
) -> CreateEmbed {
    let mut builder = CreateEmbed::new();
    if let Some(ResolvedOption {
                    value: ResolvedValue::String(command),
                    ..
                }) = _options.first()
    {
        println!("{}", command);
        match *command {
            "mem_info" => {
                builder = Manager::getUsers(ctx, guild_id).await;
            }
            "fact" => {
                builder = CreateEmbed::new();
            }
            _ => {
                builder = builder
                    .title("Hello there, Human!")
                    .description(HELP_MESSAGE);
            }
        }
    } else {
        builder = builder
            .title("Hello there, Human!")
            .description(HELP_MESSAGE);
    }
    builder
}

pub fn register() -> CreateCommand {
    CreateCommand::new("manager")
        .description("Command for managers to retrieve guild's information")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "command",
                "which information to retrieve",
            )
                .add_string_choice("Information of members", "mem_info")
                .add_string_choice("Fact", "fact"),
        )
}