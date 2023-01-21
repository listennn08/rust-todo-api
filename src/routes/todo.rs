use chrono::Utc;
use diesel::{RunQueryDsl, QueryDsl, expression_methods::ExpressionMethods};
use rocket::{serde::json::{Json, Value}, http::Status};
use serde_json::{from_str, to_string};

use crate::{db::establish_connection, models::todo::{Todo, NewTodo, UpdateTodo}};

#[get("/")]
pub fn get_all_todos() -> Value {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection();
    let results = todos
        .get_results::<Todo>(conn)
        .expect("Error loading todos");

    from_str::<Value>(to_string(&results).unwrap().as_str()).unwrap()
}

#[get("/<todo_id>")]
pub fn get_todo(todo_id: i32) -> Value {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection();
    let results = todos
        .filter(id.eq(todo_id))
        .get_result::<Todo>(conn)
        .expect("Can not get todo item");

    from_str(to_string(&results).unwrap().as_str()).unwrap()
}

#[post("/", format="json", data="<todo>")]
pub fn add_todo(todo: Json<NewTodo>) -> Status {
    use crate::schema::todos;
    use crate::models::todo::NewTodo;

    let conn = &mut establish_connection();
    let title = todo.into_inner().title;
    let new_todo = NewTodo { title };
    
    diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(conn)
            .expect("Error saving new todo");

    Status::Created
}

#[put("/<todo_id>", format="json", data="<todo>")]
pub fn update_todo(todo_id: i32, todo: Json<UpdateTodo>) -> Status {
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
