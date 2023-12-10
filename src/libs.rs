#![allow(non_snake_case)]
pub mod Handler;
pub mod Activity;
pub mod Joke;
pub mod Cat;

//for D&D
pub mod DnD;

pub const I_HELP_COMMAND: &str = "!help";
pub const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

❓ Need technical help?
➡️ Post in the <#1180454433063190629> channel and other humans will assist you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— HelpBot 🤖
";
