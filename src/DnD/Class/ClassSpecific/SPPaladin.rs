use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPPaladin {
    aura_range: i32,
}
impl SPPaladin {
    pub fn new() -> Self {
        SPPaladin {
            aura_range: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPPaladin {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}