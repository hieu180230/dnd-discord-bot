struct AbilityScore{
    index:String,
    name:String,
    url:String,
}
pub struct Skill{
    index:String,
    name:String,
    url:String,
    desc:Vec<String>,
    ability_score:AbilityScore,
}