use crate::DnD::Schemas::APIReference;

pub struct Skill{
    pub index:String,
    pub name:String,
    pub url:String,
    pub desc:Vec<String>,
    pub ability_score:APIReference,
}