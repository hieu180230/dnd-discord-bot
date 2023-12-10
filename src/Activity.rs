use serde::Deserialize;

//https://www.boredapi.com/api/activity
#[derive(Deserialize, Debug)]
struct Activity{
    _activity : String,
    _type : String,
    _participants : u16,
    _price : u16,
    _link : String,
    _key : u16,
    _accessibility : u16
}

impl Activity {
    fn toString(&self) -> String{
        format!("Activity: {act}", act=self._activity)
    }
}