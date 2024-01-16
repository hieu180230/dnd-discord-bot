use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPCleric {
    channel_divinity_charge: i32,
    destroy_undead_cr: i32,
}
impl SPCleric {
    pub fn new() -> Self {
        SPCleric {
            channel_divinity_charge: -1,
            destroy_undead_cr: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPCleric {
    async fn from_value(&mut self, json: serde_json::Value) {

    }
}
