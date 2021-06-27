extern crate reqwest;
use reqwest::{ClientBuilder, Error};
extern crate serde_json;
use super::line::meta_mode::MetaMode;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct TflClient {
  client: reqwest::Client,
  app_id: String,
  app_key: String,
}

impl TflClient {
  pub fn new() -> Result<TflClient, Error> {
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

  pub fn get_line_meta_modes(&self) -> Vec<MetaMode> {
    let body = self.get("/line/meta/modes");
    match body {
      Ok(body) => {
        let str = String::from(body);
        let meta_modes = serde_json::from_str(&str).unwrap();
        meta_modes
      }
      Err(_) => Vec::new(),
    }
  }
}
