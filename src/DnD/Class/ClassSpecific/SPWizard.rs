use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

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

    }
}