#![allow(non_snake_case)]
pub mod Handler;
pub mod Activity;
pub mod Joke;
pub mod Cat;

//for D&D
pub mod DnD;

pub const HELP_MESSAGE: &str = "
You have summoned me. Let's see about getting you what you need.

❓ Command about cat?
➡️ I got `/cat image` and `/cat fact` for you 🙏

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— HelpBot 🤖
";
