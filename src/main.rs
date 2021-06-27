#[macro_use]
extern crate rocket;

mod routes;
mod sqlx;
pub mod tlf;
use routes::{index, modes};

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(sqlx::stage())
    .mount("/", routes![index])
    .mount("/api", routes![modes])
}
