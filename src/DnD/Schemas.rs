#[allow(non_camel_case_types)]
use std::collections::HashMap;
use reqwest::Client;

use serde::Deserialize;
use serde_json::{Value, Map, from_str};
use serde_json::map::Values;

use crate::DnD::API_SERVER;


//this store the API reference of an item
#[derive(Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
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
    pub fn parse(T: &Value) -> Self{
        APIReference{
            index:T["index"].as_str().unwrap().to_string(),
            name:T["name"].as_str().unwrap().to_string(),
            url:T["url"].as_str().unwrap().to_string()
        }
    }

    pub fn display(T: Vec<APIReference>) -> String{
        let mut res: String = "".to_string();
        for t in &T {
            res += &*format!("- *{}*\n", t.name);
        }
        return res;
    }
}

//difficulty check
#[derive(Deserialize, Debug)]
enum SUCCESS_TYPE{
    None,
    Half,
    Other,
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
            success_type:SUCCESS_TYPE::None,
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
pub struct Ideal{
    pub desc:String,
    pub alignments:Vec<APIReference>
}
impl Ideal{
    pub fn new() -> Self{
        Ideal{
            desc:"".to_string(),
            alignments:vec![],
        }
    }
    pub fn parse(object: &Map<String, Value>) -> Self {
        let mut ideal = Ideal::new();
        ideal.desc = object["desc"].as_str().unwrap().to_string();
        //alignment is a vector of objects (APIReference)
        let mut alignments = object["alignments"].as_array().unwrap();
        for i in alignments {
            //get teh object, parse it in to a APIReference and push it into the alignment vector
            //of an ideal object
            let mut _reference = i.as_object().unwrap();
            let mut new_ref = APIReference::new();
            new_ref.name = _reference["name"].as_str().unwrap().to_string();
            new_ref.url = _reference["url"].as_str().unwrap().to_string();
            new_ref.index = _reference["index"].as_str().unwrap().to_string();
            ideal.alignments.push(new_ref);
        }
        return ideal;
    }
}

//Contains a reference to something else in the API along with a count.
#[derive(Deserialize, Debug)]
pub struct CountRef{
    pub count:i64,
    pub of:APIReference,
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
pub struct ScorePrerequisite{
    pub ability_score:APIReference,
    pub minimum_score:i64,
}
impl ScorePrerequisite{
    pub fn new() -> Self{
        ScorePrerequisite{
            ability_score:APIReference::new(),
            minimum_score:-1,
        }
    }
    pub fn parse(T: &Value) -> Self{
        ScorePrerequisite{
            ability_score: APIReference {
                index: T["ability_score"].as_object().unwrap()["index"].as_str().unwrap().to_string(),
                name: T["ability_score"].as_object().unwrap()["name"].as_str().unwrap().to_string(),
                url: T["ability_score"].as_object().unwrap()["url"].as_str().unwrap().to_string(),
            },
            minimum_score: T["minimum_score"].as_i64().unwrap(),
        }
    }
}

//Contains a reference to an ability score and a bonus
#[derive(Deserialize, Debug)]
struct AbilityBonus{
    pub ability_score:APIReference,
    pub bonus:i64,
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
        Choice{
            desc:"".to_string(),
            choose:-1,
            choice_type:"".to_string(),
            from:OptionSet::new(),
        }
    }
}

#[derive(Clone)]
pub struct APIReferenceList{
    pub count: i64,
    pub results: Vec<APIReference>,
}

const ENDPOINTS: &[&str] = &["ability-scores", "alignments", "backgrounds", "classes",
                                  "conditions", "damage-types", "equipment", "equipment-categories",
                                  "feats", "features", "languages", "magic-items", "magic-schools",
                                  "monsters", "proficiencies", "races", "rule-sections", "rules",
                                  "skills", "spells", "subclasses", "subraces", "traits", "weapon-properties"];
impl APIReferenceList{
    pub fn new() -> Self{
        APIReferenceList{
            count: -1,
            results: vec![],
        }
    }
    pub async fn load() -> HashMap<String, Self>{
        let mut resources: HashMap<String, APIReferenceList> = HashMap::new();
        for endpoint in ENDPOINTS{
            let client = Client::new();
            let res = client.get(format!("{}/api/{}",API_SERVER, endpoint))
                .send()
                .await
                .expect("fail to get to link")
                .text()
                .await
                .expect("fail to convert to json");
            let json:Value = from_str(&res).expect("what?");
            let mut references = APIReferenceList::new();
            references.count = json["count"].as_i64().unwrap();
            let resource_array = json["results"].as_array().unwrap();
            for resource in resource_array{
                let mut reference = APIReference::new();
                reference.name = resource["name"].as_str().unwrap().to_string();
                reference.url = resource["url"].as_str().unwrap().to_string();
                reference.index = resource["index"].as_str().unwrap().to_string();
                references.results.push(reference);
            }
            resources.insert(endpoint.to_string(), references);
        }
        resources
    }
}

