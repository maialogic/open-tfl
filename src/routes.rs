use crate::tfl::{client, line};
use client::TflClient;
use line::meta_mode::MetaMode;
use rocket::response::Debug;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {
  "Hello, Open TLF!"
}

#[get("/modes")] // <- route attribute
pub async fn modes() -> Result<Json<Vec<MetaMode>>, Debug<Box<dyn std::error::Error>>> {
  let client = TflClient::new()?;
  let result = client.get_line_meta_modes().await?;
  Ok(Json(result))
}
