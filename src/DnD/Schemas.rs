use serde::Deserialize;
use serde_json::Map;
use serenity::async_trait;

//this store the API reference of an item
#[derive(Deserialize, Debug, Eq, Hash, PartialEq)]
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
    dc_value:i64,
    success_type:SUCCESS_TYPE,
}
impl DC{
    pub fn new() -> Self{
        DC{
            dc_type:APIReference::new(),
            dc_value:-1,
            success_type:SUCCESS_TYPE::none,
        }
    }
}


//this store the type of damage and how much damage it deals
#[derive(Deserialize, Debug)]
pub struct Damage{
    damage_type:APIReference,
    damage_dice:String,
}
impl Damage{
    pub fn new() -> Self{
        Damage{
            damage_type:APIReference::new(),
            damage_dice:"".to_string(),
        }
    }
}

//Opiton and its structs

//Contains information describing an action, for use within multiple attacks actions and/or attack action.
#[derive(Deserialize, Debug)]
enum ATTACK_TYPE{
    melee,
    ranged,
    ability,
    magic,
    none,
}
#[derive(Deserialize, Debug)]
struct Action{
    action_name:String,
    count:i64,
    action_type:ATTACK_TYPE,
}
impl Action{
    pub fn new() -> Self{
        Action{
            action_name:"".to_string(),
            count:-1,
            action_type:ATTACK_TYPE::none,
        }
    }
}

//Contains information about an ideal (part of character definition)
#[derive(Deserialize, Debug)]
struct Ideal{
    desc:String,
    alignments:Vec<APIReference>
}
impl Ideal{
    pub fn new() -> Self{
        Ideal{
            desc:"".to_string(),
            alignments:vec![APIReference::new()],
        }
    }
}

//Contains a reference to something else in the API along with a count.
#[derive(Deserialize, Debug)]
struct CountRef{
    count:i64,
    of:APIReference,
}
impl CountRef{
    pub fn new() -> Self{
        CountRef{
            count:-1,
            of:APIReference::new(),
        }
    }
}

//Contains a reference to an ability score and a minimum score.
#[derive(Deserialize, Debug)]
struct ScorePrerequisite{
    ability_score:APIReference,
    minimum_score:i64,
}
impl ScorePrerequisite{
    pub fn new() -> Self{
        ScorePrerequisite{
            ability_score:APIReference::new(),
            minimum_score:-1,
        }
    }
}

//Contains a reference to an ability score and a bonus
#[derive(Deserialize, Debug)]
struct AbilityBonus{
    ability_score:APIReference,
    bonus:i64,
}
impl AbilityBonus{
    pub fn new() -> Self{
        AbilityBonus{
            ability_score:APIReference::new(),
            bonus:-1,
        }
    }
}

//Contains a reference to information about a breath attack. Breath stinky damage ??
#[derive(Deserialize, Debug)]
struct Breath{
    name:String,
    dc:DC,
    damage:Damage,
}
impl Breath{
    pub fn new() -> Self{
        Breath{
            name:"".to_string(),
            dc:DC::new(),
            damage:Damage::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Option{
    pub option_type:String,
    pub reference:APIReference,
    pub action:Action,
    pub multiple:Vec<Option>, //if multiple actions are chosen
    pub choice:Choice, //A nested choice
    pub string:String,
    pub ideal:Ideal,
    pub counted_reference:CountRef,
    pub score_prerequisite:ScorePrerequisite,
    pub ability_bonus:AbilityBonus,
    pub breath:Breath,
    pub damage:Damage,
}
impl Option{
    pub fn new() -> Self{
        print!("option");
        Option{
            option_type:"".to_string(),
            reference:APIReference::new(),
            action:Action::new(),
            multiple:vec![],
            choice:Choice::new(),
            string:"".to_string(),
            ideal:Ideal::new(),
            counted_reference:CountRef::new(),
            score_prerequisite:ScorePrerequisite::new(),
            ability_bonus:AbilityBonus::new(),
            breath:Breath::new(),
            damage:Damage::new(),
        }
    }
}


//OptionSet provide options to be chosen from or data to interpret the option
#[derive(Deserialize, Debug)]
pub struct OptionSet{
    pub option_set_type:String,
    pub options:Vec<Option>,
    pub equipment_category:APIReference,
    pub resource_list_url:String,
}
impl OptionSet{
    pub fn new() -> Self{
        print!("option set");
        OptionSet{
            option_set_type:"".to_string(),
            options:vec![],
            equipment_category:APIReference::new(),
            resource_list_url:"".to_string(),
        }
    }
}
//a choice made by a player. Commonly seen related to decisions made during character creation or combat
#[derive(Deserialize, Debug)]
pub struct Choice{
    pub desc:String,
    pub choose:i64,
    pub choice_type:String,
    pub from:OptionSet,
}
impl Choice{
    pub fn new() -> Self{
        print!("choice");
        Choice{
            desc:"".to_string(),
            choose:-1,
            choice_type:"".to_string(),
            from:OptionSet::new(),
        }
    }
}

impl Choice {
    pub async fn parse(&mut self, json:&serde_json::Value){
        //get description of the choice that players are about to make
        match json.get("desc"){
            Some(T) => {
                self.desc = json["desc"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        //get how many choices players can make
        match json.get("choose"){
            Some(T) => {
                self.choose = json["choose"].as_i64().unwrap();
            },
            None => print!("?"),
        }
        //get the type of choices
        match json.get("type"){
            Some(T) => {
                self.choice_type = json["type"].as_str().unwrap().to_string();
            },
            None => print!("?"),
        }
        //get options to choose from
        let mut from: &Map<String,serde_json::Value> = &Map::new();
        match json.get("from"){
            Some(T) => {
                from = json["from"].as_object().unwrap();
            }
            None => print!("?"),
        }
        //the temporary option set
        let mut _option_set: OptionSet = OptionSet::new();
        if Some(from["option_set_type"].as_str().unwrap()).is_some()
        {
            //get the type of this option set
            _option_set.option_set_type = from["option_set_type"].as_str().unwrap().to_string();
            //switch the type the the appropriate case. We have an array of options,
            // a category of equipment and a resource list
            match from["option_set_type"].as_str().unwrap(){
                // array of options case
                "options_array" => {
                    //get the array from API as an array of Value
                    let array = from["options"].as_array().unwrap();
                    //iterate through the array
                    for a in array{
                        //get the option, turn it into an object then switch object_type
                        // to get appropriate of Option
                        let mut _option:Option = Option::new();
                        let a_object = a.as_object().unwrap();
                        //parse in the object type
                        _option.option_type = a_object["option_type"].as_str().unwrap().to_string();
                        //get object
                        match &*_option.option_type{
                            //process if the option type is String
                            "string" => {
                                _option.string = a_object["string"].as_str().unwrap().to_string();
                            }
                            //process if the option type is Ideal
                            "ideal" => {
                                _option.ideal.desc = a_object["desc"].as_str().unwrap().to_string();
                                let mut alignments = a_object["alignments"].as_array().unwrap();
                                for i in alignments{
                                    let mut _reference = i.as_object().unwrap();
                                    let mut  new_ref = APIReference::new();
                                    new_ref.name =_reference["name"].as_str().unwrap().to_string();
                                    new_ref.url =_reference["url"].as_str().unwrap().to_string();
                                    new_ref.index =_reference["index"].as_str().unwrap().to_string();
                                    _option.ideal.alignments.push(new_ref);
                                }
                            }
                            _ => print!("?"),
                        }
                        //push each of the options to option set
                        _option_set.options.push(_option);
                    }
                }
                //equipment category case
                "equipment_category" => {
                    let _equip_object = from["equipment_category"].as_object().unwrap(); //api information
                    _option_set.equipment_category.name = _equip_object["name"].as_str().unwrap().to_string();
                    _option_set.equipment_category.index = _equip_object["index"].as_str().unwrap().to_string();
                    _option_set.equipment_category.url = _equip_object["url"].as_str().unwrap().to_string();
                }
                //resource list case
                "resource_list" => {
                    _option_set.resource_list_url = from["resource_list_url"].as_str().unwrap().to_string();
                }
                _ => print!("?"),
            }
        }
        self.from = _option_set;
    }

}

