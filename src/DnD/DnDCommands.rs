use serenity::all::standard::macros;
use serenity::framework::standard::*;
use crate::DnD::CharData::AbilityScore::*;

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
#[commands(lookup)]
struct DnD;
