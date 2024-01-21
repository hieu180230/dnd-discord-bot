use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

pub struct SPPaladin {
    aura_range: i32,
}
impl SPPaladin {
    pub fn new() -> Self {
        SPPaladin { aura_range: -1 }
    }
}
#[async_trait]
impl SPConvert for SPPaladin {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.aura_range = json["aura_range"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Aura Range: {}", self.aura_range)
    }
}
