use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

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
        self.wild_shape_max_cr = json["wild_shape_max_cr"].as_i64().unwrap() as i32;
        self.wild_shape_swim = json["wild_shape_swim"].as_bool().unwrap();
        self.wild_shape_fly = json["wild_shape_fly"].as_bool().unwrap();
    }
    fn display(&self) -> String {
        format!(
            "Wild Shape Max CR: {}\nWild Shape Swim: {}\nWild Shape Fly: {}",
            self.wild_shape_max_cr, self.wild_shape_swim, self.wild_shape_fly
        )
    }
}
