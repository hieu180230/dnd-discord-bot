use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPWarlock {
    invocations_known: i32,
    mystic_arcanum_level_6: i32,
    mystic_arcanum_level_7: i32,
    mystic_arcanum_level_8: i32,
    mystic_arcanum_level_9: i32,
}
impl SPWarlock {
    pub fn new() -> Self {
        SPWarlock {
            invocations_known: -1,
            mystic_arcanum_level_6: -1,
            mystic_arcanum_level_7: -1,
            mystic_arcanum_level_8: -1,
            mystic_arcanum_level_9: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPWarlock {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.invocations_known = json["invocations_known"].as_i64().unwrap() as i32;
        self.mystic_arcanum_level_6 = json["mystic_arcanum_level_6"].as_i64().unwrap() as i32;
        self.mystic_arcanum_level_7 = json["mystic_arcanum_level_7"].as_i64().unwrap() as i32;
        self.mystic_arcanum_level_8 = json["mystic_arcanum_level_8"].as_i64().unwrap() as i32;
        self.mystic_arcanum_level_9 = json["mystic_arcanum_level_9"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Invocations Known: {}\nMystic Arcanum Level 6: {}\nMystic Arcanum Level 7: {}\nMystic Arcanum Level 8: {}\nMystic Arcanum Level 9: {}", self.invocations_known, self.mystic_arcanum_level_6, self.mystic_arcanum_level_7, self.mystic_arcanum_level_8, self.mystic_arcanum_level_9)
    }
}