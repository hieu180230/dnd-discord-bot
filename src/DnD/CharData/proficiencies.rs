use crate::DnD::Schemas::APIReference;

pub struct Proficiencies{
    pub reference:APIReference,
    pub proficiencies_type:String,
    pub classes:Vec<APIReference>,
    pub races:Vec<APIReference>,
    pub references:Vec<APIReference>,
}