use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

struct SpellSlot {
    spell_slot_level: i32,
    sorcery_point_cost: i32,
}
pub struct SPSorcerer {
    sorcery_points: i32,
    metamagic_known: i32,
    creating_spell_slots: Vec<SpellSlot>,
}
impl SPSorcerer {
    pub fn new() -> Self {
        SPSorcerer {
            sorcery_points: -1,
            metamagic_known: -1,
            creating_spell_slots: vec![],
        }
    }
}
#[async_trait]
impl SPConvert for SPSorcerer {
    async fn from_value(&mut self, json: serde_json::Value) {

    }

}