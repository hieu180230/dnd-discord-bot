#![allow(non_snake_case)]

pub mod Activity;
pub mod Cat;
pub mod Handler;
pub mod Joke;
//for D&D
pub mod DnD;

pub mod Manager;

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
