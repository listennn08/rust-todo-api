#[macro_use] extern crate rocket;

mod routes;

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
