use serenity::async_trait;

pub mod AbilityScore;
pub mod Skills;
pub mod Alignments;
pub mod Background;
pub mod Language;
pub mod proficiencies;

#[async_trait]
pub trait Convert{
    async fn from_value(&mut self, json:serde_json::Value){}
}



