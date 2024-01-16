use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::{SPConvert};

pub struct SPBarbarian {
    rage_count: i32,
    rage_damage_bonus: i32,
    brutal_crit_dice: i32,
}
impl SPBarbarian {
    pub fn new() -> Self {
        SPBarbarian {
            rage_count: -1,
            rage_damage_bonus: -1,
            brutal_crit_dice: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPBarbarian {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.rage_count = json["rage_count"].as_i64().unwrap() as i32;
        self.rage_damage_bonus = json["rage_damage_bonus"].as_i64().unwrap() as i32;
        self.brutal_crit_dice = json["brutal_critical_dice"].as_i64().unwrap() as i32;
    }
}