use crate::DnD::Schemas::APIReference;

pub struct Proficiencies{
    reference:APIReference,
    proficiencies_type:String,
    classes:Vec<APIReference>,
    races:Vec<APIReference>,
    references:Vec<APIReference>,
}