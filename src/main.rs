#[macro_use]
extern crate rocket;

mod routes;
mod db;
pub mod tfl;
use routes::{index, modes};
use tfl::db as tfl_db;

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(db::stage())
    .attach(tfl_db::stage())
    .mount("/", routes![index])
    .mount("/api", routes![modes])
}
