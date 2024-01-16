use std::collections::HashMap;
use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

struct MartialArt {
    dice_count: i32,
    dice_value: i32,
}

pub struct SPMonk {
    ki_points: i32,
    unarmored_movement: i32,
    martial_art: MartialArt
}

impl SPMonk {
    pub fn new() -> Self {
        SPMonk {
            ki_points: -1,
            unarmored_movement: -1,
            martial_art: MartialArt {
                dice_count: -1,
                dice_value: -1,
            }
        }
    }
}
#[async_trait]
impl SPConvert for SPMonk {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}