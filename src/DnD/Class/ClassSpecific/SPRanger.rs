use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPRanger {
    favoured_enemies: i32,
    favoured_terrain: i32,
}
impl SPRanger {
    pub fn new() -> Self {
        SPRanger {
            favoured_enemies: -1,
            favoured_terrain: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPRanger {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}