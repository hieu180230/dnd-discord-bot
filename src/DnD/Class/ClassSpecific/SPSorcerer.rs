use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

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
        self.sorcery_points = json["sorcery_points"].as_i64().unwrap() as i32;
        self.metamagic_known = json["metamagic_known"].as_i64().unwrap() as i32;
        for spell_slot in json["creating_spell_slots"].as_array().unwrap() {
            self.creating_spell_slots.push(SpellSlot {
                spell_slot_level: spell_slot["spell_slot_level"].as_i64().unwrap() as i32,
                sorcery_point_cost: spell_slot["sorcery_point_cost"].as_i64().unwrap() as i32,
            });
        }
    }
    fn display(&self) -> String {
        let mut spell_slots = String::new();
        for spell_slot in &self.creating_spell_slots {
            spell_slots.push_str(&format!(
                "- Spell Slot Level: {}\n- Sorcery Point Cost: {}\n",
                spell_slot.spell_slot_level, spell_slot.sorcery_point_cost
            ));
        }
        format!(
            "Sorcery Points: {}\nMetamagic Known: {}\nCreating Spell Slots:\n{}",
            self.sorcery_points, self.metamagic_known, spell_slots
        )
    }
}
