

use serde::de::{Deserialize};
use serde_json;
use serde_json::error::{Error};

pub fn deserialise<'de, D: Deserialize<'de>>(string: &'de str) -> Result<D, Error> {
    serde_json::from_str::<'de>(string)
}
