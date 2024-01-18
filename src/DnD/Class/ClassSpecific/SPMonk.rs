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
        self.ki_points = json["ki_points"].as_i64().unwrap() as i32;
        self.unarmored_movement = json["unarmored_movement"].as_i64().unwrap() as i32;
        self.martial_art.dice_count = json["martial_arts"].as_object().unwrap()["dice_count"].as_i64().unwrap() as i32;
        self.martial_art.dice_value = json["martial_arts"].as_object().unwrap()["dice_value"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Ki Points: {}\nUnarmored Movement: {}\nMartial Arts: {}d{}", self.ki_points, self.unarmored_movement, self.martial_art.dice_count, self.martial_art.dice_value)
    }
}