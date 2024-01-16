use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPDruid {
    wild_shape_max_cr: i32,
    wild_shape_swim: bool,
    wild_shape_fly: bool,
}
impl SPDruid {
    pub fn new() -> Self {
        SPDruid {
            wild_shape_max_cr: -1,
            wild_shape_swim: false,
            wild_shape_fly: false,
        }
    }
}
#[async_trait]
impl SPConvert for SPDruid {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}