#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::tokio::task::spawn_blocking;

mod tlf;
use tlf::client::TflClient;
use tlf::line::meta_mode::MetaMode;

#[get("/")]
fn index() -> &'static str {
  "Hello, Open TLF!"
}

#[get("/modes")] // <- route attribute
async fn modes() -> Json<Vec<MetaMode>> {
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

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
    .mount("/api", routes![modes])
}
