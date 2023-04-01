#[macro_use] extern crate rocket;

mod routes;
pub mod models;
pub mod schema;
pub mod db;
pub mod utils;
pub mod middleware;

#[get("/ping")]
fn health_check() -> &'static str {
    "pong"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_check])
        .attach(routes::todo::stage())
        .attach(routes::auth::stage())
}
