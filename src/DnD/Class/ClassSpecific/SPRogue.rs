use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

struct SneakAttack {
    dice_count: i32,
    dice_value: i32,
}
pub struct SPRogue {
    sneak_attack: SneakAttack,
}
impl SPRogue {
    pub fn new() -> Self {
        SPRogue {
            sneak_attack: SneakAttack {
                dice_count: -1,
                dice_value: -1,
            }
        }
    }
}
#[async_trait]
impl SPConvert for SPRogue {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.sneak_attack.dice_count = json["sneak_attack"].as_object().unwrap()["dice_count"].as_i64().unwrap() as i32;
        self.sneak_attack.dice_value = json["sneak_attack"].as_object().unwrap()["dice_value"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Sneak Attack: {}d{}", self.sneak_attack.dice_count, self.sneak_attack.dice_value)
    }
}