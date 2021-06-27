use rocket::serde::json::Json;
use rocket::tokio::task::spawn_blocking;

use crate::tlf::{client, line};
use client::TflClient;
use line::meta_mode::MetaMode;

#[get("/")]
pub fn index() -> &'static str {
  "Hello, Open TLF!"
}

#[get("/modes")] // <- route attribute
pub async fn modes() -> Json<Vec<MetaMode>> {
  let client = TflClient::new();
  match client {
    Ok(c) => {
      let result = spawn_blocking(move || c.get_line_meta_modes()).await;
      match result {
        Ok(res) => Json(res),
        Err(_) => Json(Vec::new()),
      }
    }
    Err(_) => Json(Vec::new()),
  }
}
