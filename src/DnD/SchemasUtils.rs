use crate::DnD::Schemas::*;
use serde_json::Map;
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
                        let a_object = a.as_object().unwrap(); //an object in the array
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
                                //alignment is a vector of objects (APIReference)
                                let mut alignments = a_object["alignments"].as_array().unwrap();
                                for i in alignments{
                                    //get teh object, parse it in to a APIReference and push it into the alignment vector
                                    //of an ideal object
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