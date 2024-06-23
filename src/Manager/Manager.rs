use serenity::all::{Context, Message};
use serenity::all::standard::Args;
use serenity::async_trait;
use serenity::framework::standard::CommandResult;
use serenity::model::{user, guild};

pub struct Manager
{

}

pub struct Guild;

#[async_trait]
impl Manager
{
    pub async fn getUsers(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
        let guild_id = msg.guild_id;
        let members = guild_id.members(&ctx.http, None, None);
        let mut reply:String = String::new();
        match members{
            Ok(T) => {
                for member in T{
                    reply += &*format!("{}\n", member.user.name);
                }

            }
            Err(E) => {
                reply = &*format!("Err: {}\n", E);
            }
        }
        msg.reply(ctx, reply).await?;
        Ok(())
    }
}