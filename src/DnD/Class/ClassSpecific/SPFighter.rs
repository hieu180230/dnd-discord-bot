use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

pub struct SPFighter {
    action_surges: i32,
    indomitable_uses: i32,
    extra_attacks: i32,
}
impl SPFighter {
    pub fn new() -> Self {
        SPFighter {
            action_surges: -1,
            indomitable_uses: -1,
            extra_attacks: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPFighter {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.action_surges = json["action_surges"].as_i64().unwrap() as i32;
        self.indomitable_uses = json["indomitable_uses"].as_i64().unwrap() as i32;
        self.extra_attacks = json["extra_attacks"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!(
            "Action Surges: {}\nIndomitable Uses: {}\nExtra Attacks: {}",
            self.action_surges, self.indomitable_uses, self.extra_attacks
        )
    }
}
