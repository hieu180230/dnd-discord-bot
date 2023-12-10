use crate::DnD::Schemas::*;
struct Feature{
    feature_type:String,
    desc:String,
}
pub struct Background{
    reference:APIReference,
    starting_proficiencies:Vec<APIReference>,
    starting_equipment:Vec<APIReference>,
    starting_equipment_options:Choice,
    language_option:Choice,
    feature:Feature,
    personality:Choice,
    ideals:Choice,
    bonds:Choice,
    flaws:Choice,
}