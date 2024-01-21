use crate::DnD::Class::ClassSpecific::*;
use serenity::async_trait;

#[async_trait]
pub trait SPConvert {
    async fn from_value(&mut self, json: serde_json::Value);
    fn display(&self) -> String;
}

pub enum ClassType {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}
pub struct SPFactory;
impl SPFactory {
    pub fn new(class: &ClassType) -> Box<dyn SPConvert + Send> {
        match class {
            ClassType::Barbarian => Box::new(SPBarbarian::SPBarbarian::new()),
            ClassType::Bard => Box::new(SPBard::SPBard::new()),
            ClassType::Cleric => Box::new(SPCleric::SPCleric::new()),
            ClassType::Druid => Box::new(SPDruid::SPDruid::new()),
            ClassType::Fighter => Box::new(SPFighter::SPFighter::new()),
            ClassType::Monk => Box::new(SPMonk::SPMonk::new()),
            ClassType::Paladin => Box::new(SPPaladin::SPPaladin::new()),
            ClassType::Ranger => Box::new(SPRanger::SPRanger::new()),
            ClassType::Rogue => Box::new(SPRogue::SPRogue::new()),
            ClassType::Sorcerer => Box::new(SPSorcerer::SPSorcerer::new()),
            ClassType::Warlock => Box::new(SPWarlock::SPWarlock::new()),
            ClassType::Wizard => Box::new(SPWizard::SPWizard::new()),
        }
    }
}
