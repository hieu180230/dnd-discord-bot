use crate::DnD::Convert;
use serenity::async_trait;

pub enum GameMechanicType {
    Condition(&'static str),
    DamageType(&'static str),
    MagicSchool(&'static str),
    None(&'static str),
}

pub struct GameMechanic {
    pub GameMechanicType: GameMechanicType,
    pub index: String,
    pub name: String,
    pub url: String,
    pub desc: Vec<String>,
}

impl GameMechanic {
    pub fn new() -> Self {
        GameMechanic {
            index: String::new(),
            name: String::new(),
            url: String::new(),
            desc: vec![],
            GameMechanicType: GameMechanicType::None(""),
        }
    }
}
#[async_trait]
impl Convert for GameMechanic {
    async fn from_value(&mut self, json: serde_json::Value) {
        match json.get("index") {
            Some(T) => {
                self.index = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No index found")
            }
        }
        match json.get("name") {
            Some(T) => {
                self.name = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No name found")
            }
        }
        match json.get("url") {
            Some(T) => {
                self.url = T.as_str().unwrap().to_string();
            }
            None => {
                print!("No url found")
            }
        }
        match json.get("desc") {
            Some(T) => {
                for i in T.as_array().unwrap() {
                    self.desc.push(i.as_str().unwrap().to_string());
                }
            }
            None => {
                print!("No desc found")
            }
        }
    }
}
