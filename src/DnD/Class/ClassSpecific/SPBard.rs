use serenity::async_trait;
use crate::DnD::Class::ClassSpecific::SPFactory::SPConvert;

pub struct SPBard {
    bardic_inspiration_dice: i32,
    song_of_rest_die: i32,
    magical_secrets_max_5: i32,
    magical_secrets_max_7: i32,
    magical_secrets_max_9: i32,
}
impl SPBard {
    pub fn new() -> Self {
        SPBard {
            bardic_inspiration_dice: -1,
            song_of_rest_die: -1,
            magical_secrets_max_5: -1,
            magical_secrets_max_7: -1,
            magical_secrets_max_9: -1,
        }
    }
}
#[async_trait]
impl SPConvert for SPBard {
    async fn from_value(&mut self, json: serde_json::Value) {
        self.bardic_inspiration_dice = json["bardic_inspiration_die"].as_i64().unwrap() as i32;
        self.song_of_rest_die = json["song_of_rest_die"].as_i64().unwrap() as i32;
        self.magical_secrets_max_5 = json["magical_secrets_max_5"].as_i64().unwrap() as i32;
        self.magical_secrets_max_7 = json["magical_secrets_max_7"].as_i64().unwrap() as i32;
        self.magical_secrets_max_9 = json["magical_secrets_max_9"].as_i64().unwrap() as i32;
    }
    fn display(&self) -> String {
        format!("Bardic Inspiration Dice: {}\nSong of Rest Die: {}\nMagical Secrets Max 5: {}\nMagical Secrets Max 7: {}\nMagical Secrets Max 9: {}",
                self.bardic_inspiration_dice, self.song_of_rest_die, self.magical_secrets_max_5, self.magical_secrets_max_7, self.magical_secrets_max_9)
    }
}
