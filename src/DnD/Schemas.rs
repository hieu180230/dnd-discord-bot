use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APIReference{
    pub index:String,
    pub name:String,
    pub url:String,
}
impl APIReference{
    pub fn new() -> Self{
        APIReference{
            index:"".to_string(),
            name:"".to_string(),
            url:"".to_string(),
        }
    }
}
//difficulty check
#[derive(Deserialize, Debug)]
enum SUCCESS_TYPE{
    none,
    half,
    other,
}
#[derive(Deserialize, Debug)]
pub struct DC{
    dc_type:APIReference,
    dc_value:i32,
    success_type:SUCCESS_TYPE,
}

//damage
#[derive(Deserialize, Debug)]
pub struct Damage{
    damage_type:APIReference,
    damage_dice:String,
}

//Opiton and its structs
#[derive(Deserialize, Debug)]
enum ATTACK_TYPE{
    melee,
    ranged,
    ability,
    magic,
}
#[derive(Deserialize, Debug)]
struct Action{
    action_name:String,
    count:i32,
    action_type:ATTACK_TYPE,
}//Contains information describing an action, for use within Multiattack actions.
#[derive(Deserialize, Debug)]
struct Ideal{
    desc:String,
    alignments:Vec<APIReference>
}//Contains information about an ideal.
#[derive(Deserialize, Debug)]
struct CountRef{
    count:i32,
    of:APIReference,
}//Contains a reference to something else in the API along with a count.
#[derive(Deserialize, Debug)]
struct ScorePrerequisite{
    ability_score:APIReference,
    minimum_score:i32,
}//Contains a reference to an ability score and a minimum score.
#[derive(Deserialize, Debug)]
struct AbilityBonus{
    ability_score:APIReference,
    bonus:i32,
}//Contains a reference to an ability score and a bonus
#[derive(Deserialize, Debug)]
struct Breath{
    name:String,
    dc:DC,
    damage:Damage,
}//Contains a reference to information about a breath attack.
#[derive(Deserialize, Debug)]
pub struct Option{
    option_type:String,
    reference:APIReference,
    action:Action,
    multiple:Vec<Option>, //if multiple actions are chosen
    choice:Choice, //A nested choice
    string:String,
    ideal:Ideal,
    counted_reference:CountRef,
    score_prerequisite:ScorePrerequisite,
    ability_bonus:AbilityBonus,
    breath:Breath,
    damage:Damage,
}

//OptionSet provide options to be chosen from or data to interpret the option
#[derive(Deserialize, Debug)]
pub struct OptionSet{
    option_set_type:String,
    options:Vec<Option>,
    equipment_category:APIReference,
    resource_list_url:String,
}

//a choice made by a player. Commonly seen related to decisions made during character creation or combat
#[derive(Deserialize, Debug)]
pub struct Choice{
    desc:String,
    choose:i32,
    choice_type:String,
    from:OptionSet,
}

