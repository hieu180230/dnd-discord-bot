use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPBard {
    bardic_inspiration_dice: i32,
    song_of_rest_die: i32,
    magical_secrets_max_5: i32,
    magical_secrets_max_7: i32,
    magical_secrets_max_9: i32,
}
impl SPBard {
    pub fn new() -> Self {
        SPBard {
            bardic_inspiration_dice: -1,
            song_of_rest_die: -1,
            magical_secrets_max_5: -1,
            magical_secrets_max_7: -1,
            magical_secrets_max_9: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPBard {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}
