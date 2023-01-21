#[macro_use] extern crate rocket;

mod routes;
pub mod models;
pub mod schema;
pub mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index])
        .mount("/api/todo", routes![
            routes::todo::get_all_todos,
            routes::todo::get_todo,
            routes::todo::add_todo,
            routes::todo::update_todo,
        ])
}
