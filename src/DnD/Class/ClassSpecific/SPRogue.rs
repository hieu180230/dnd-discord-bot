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

    }
}