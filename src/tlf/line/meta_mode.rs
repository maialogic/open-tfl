use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "rocket::serde")]
pub struct MetaMode {
  #[serde(rename(serialize = "apiTypes", deserialize = "$type"))]
  api_types: String,
  #[serde(rename = "isTflService")]
  is_tfl_service: bool,
  #[serde(rename = "isFarePaying")]
  is_fare_paying: bool,
  #[serde(rename = "isScheduledService")]
  is_scheduled_service: bool,
  #[serde(rename = "modeName")]
  mode_name: String,
}
