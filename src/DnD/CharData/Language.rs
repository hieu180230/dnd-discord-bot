use crate::DnD::Schemas::APIReference;

enum LANGUAGE_TYPE{
    Standard,
    Exotic,
}
pub struct Language{
    pub reference:APIReference,
    pub desc:String,
    pub language_type:LANGUAGE_TYPE,
    pub script:String,
    pub typical_speaker:Vec<String>,
}