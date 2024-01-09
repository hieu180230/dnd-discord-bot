//! This fetches a random activity from an API.
//! This is an Activity API Fetcher
//!  - Doing something
//!  - Doing something else

use serde::Deserialize;

/// This fetches a random activity from an API
/// #This is an Activity API Fetcher Struct
///  - Doing something (https://www.boredapi.com/api/activity)
///   - Doing something else
#[derive(Deserialize, Debug)]
pub struct Activity {
    _activity: String,
    _type: String,
    _participants: u16,
    _price: u16,
    _link: String,
    _key: u16,
    _accessibility: u16,
}

impl Activity {
    pub fn display(&self) -> String {
        format!("Activity: {act}", act = self._activity)
    }
}
