use serde::Deserialize;

//https://official-joke-api.appspot.com/random_joke
#[derive(Deserialize)]
struct Joke{
    _type : String,
    _setup : String,
    _punch : String,
    _id : u16
}