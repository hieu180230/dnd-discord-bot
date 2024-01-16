use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPFighter {
    action_surges: i32,
    indomitable_uses: i32,
    extra_attacks: i32,
}
impl SPFighter {
    pub fn new() -> Self {
        SPFighter {
            action_surges: -1,
            indomitable_uses: -1,
            extra_attacks: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPFighter {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}