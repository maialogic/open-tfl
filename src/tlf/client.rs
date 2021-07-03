extern crate reqwest;
use super::line::meta_mode::MetaMode;
use reqwest::{ClientBuilder, Error};
use rocket::serde::json;
use rocket::tokio::task::spawn_blocking;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct TflClient {
  client: reqwest::Client,
  app_id: String,
  app_key: String,
}

impl TflClient {
  pub fn new() -> Result<TflClient, Box<dyn std::error::Error>> {
    let tfl_client = TflClient {
      client: ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()?,
      app_id: String::new(),
      app_key: String::new(),
    };

    Ok(tfl_client)
  }

  #[tokio::main]
  pub async fn get(&self, endpoint: &str) -> Result<String, Error> {
    let req_uri = format!(
      "https://api.tfl.gov.uk{}?app_id={}&app_key={}",
      endpoint, self.app_id, self.app_key
    );
    let res = self.client.get(&req_uri).send().await?.text().await?;
    Ok(res)
  }

  pub async fn get_line_meta_modes(self) -> Result<Vec<MetaMode>, Box<dyn std::error::Error>> {
    let body = spawn_blocking(move || self.get("/line/meta/modes")).await?;
    let str = String::from(body?);
    let meta_modes: Vec<MetaMode> = json::from_str(&str).unwrap();
    Ok(meta_modes)
  }
}
