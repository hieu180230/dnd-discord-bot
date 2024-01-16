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

    }
}