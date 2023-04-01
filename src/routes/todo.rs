use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;
use rocket::{serde::json::{Json, Value, to_value}, http::Status, fairing::AdHoc};
use serde::Deserialize;

use crate::db::establish_connection;
use crate::models::todo::{Todo, NewTodo, UpdateTodo};
use crate::middleware::auth::AuthMiddleware;

#[get("/", format = "json")]
async fn get_all_todos(auth: &AuthMiddleware) -> Value {
    use crate::schema::todos::dsl::*;

    // get user info from middleware
    let user_info = auth.0.clone();

    let conn = &mut establish_connection();
    let mut query = todos.into_boxed();

    if user_info.role != "admin" {
        query = query.filter(id.eq(user_info.id));
    }

    let results = query.load::<Todo>(conn).expect("Error: loading todos failure");

    to_value(&results).unwrap()
}

#[get("/<todo_id>", format = "json")]
fn get_todo(_auth: &AuthMiddleware, todo_id: i32) -> Value {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection();
    let results = todos
        .filter(id.eq(todo_id))
        .get_result::<Todo>(conn)
        .expect("Error: loading todo item failure");

    to_value(&results).unwrap()
}

#[derive(Deserialize)]
struct TodoPayload {
    title: String
}

#[post("/", format="json", data="<todo>")]
fn add_todo(auth: &AuthMiddleware, todo: Json<TodoPayload>) -> Status {
    use crate::schema::todos;

    let conn = &mut establish_connection();
    let new_todo = NewTodo {
        title: &todo.0.title,
        created_by: auth.0.id,
    };

    diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(conn)
            .expect("Error: saving new todo failure");

    Status::Created
}

#[put("/<todo_id>", format="json", data="<todo>")]
fn update_todo(todo_id: i32, todo: Json<UpdateTodo>) -> Status {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(todos)
        .filter(id.eq(todo_id))
        .set((
            title.eq(todo.title),
            done.eq(todo.done),
            updated_at.eq(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        ))
        .execute(conn)
        .expect("Update todo error");

    Status::Ok
}

#[delete("/<todo_id>")]
fn delete_todo(auth: &AuthMiddleware, todo_id: i32) -> Status {
    use crate::schema::todos::dsl::*;

    let user_info = auth.0.clone();

    let mut query = todos.into_boxed().filter(id.eq(todo_id));

    if user_info.role != "admin" {
        query = query.filter(created_by.eq(user_info.id))
    }

    let conn = &mut establish_connection();
    let result = query.first::<Todo>(conn);

    return match result {
        Ok(_) => {
            diesel::delete(todos)
                .filter(id.eq(todo_id))
                .execute(conn)
                .expect("Error: execute delete todo item failure");
            return Status::Ok;
        },
        Err(diesel::result::Error::NotFound) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}

// create a group route and mount to server with middleware
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Todo", |rocket| async {
        rocket
            .mount("/api/todo", routes![
                get_all_todos,
                get_todo,
                add_todo,
                update_todo,
                delete_todo,
            ])
    })
} 
