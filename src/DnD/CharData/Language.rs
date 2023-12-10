use crate::DnD::Schemas::APIReference;

enum LANGUAGE_TYPE{
    Standard,
    Exotic,
}
pub struct Language{
    reference:APIReference,
    desc:String,
    language_type:LANGUAGE_TYPE,
    script:String,
    typical_speaker:Vec<String>,
}