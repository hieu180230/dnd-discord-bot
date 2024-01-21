use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;
use serenity::async_trait;

pub struct SPRanger {
    favoured_enemies: i32,
    favoured_terrain: i32,
}
impl SPRanger {
    pub fn new() -> Self {
        SPRanger {
            favoured_enemies: -1,
            favoured_terrain: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPRanger {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.favoured_enemies = json["favoured_enemies"].as_i64().unwrap() as i32;
        self.favoured_terrain = json["favoured_terrain"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!(
            "Favoured Enemies: {}\nFavoured Terrain: {}",
            self.favoured_enemies, self.favoured_terrain
        )
    }
}
