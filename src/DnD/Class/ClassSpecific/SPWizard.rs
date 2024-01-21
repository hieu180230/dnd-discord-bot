use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

pub struct SPWizard {
    arcane_recover_levels: i32,
}
impl SPWizard {
    pub fn new() -> Self {
        SPWizard {
            arcane_recover_levels: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPWizard {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.arcane_recover_levels = json["arcane_recover_levels"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Arcane Recover Levels: {}", self.arcane_recover_levels)
    }
}
