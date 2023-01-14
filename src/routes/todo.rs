use either::*;
use std::fs::read_to_string;
use chrono::Utc;
use rocket::{serde::json::{json, Json, Value}, http::Status};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Deserialize, Serialize)]
struct Database {
    todos: Vec<Todo>
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool,
    pub created: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub done: Option<bool>,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            title,
            done: false,
            created: Utc::now().timestamp()
        }
    }
}

#[derive(Deserialize)]
pub struct NewTodo {
    title: String
}

#[get("/")]
pub fn get_all_todos() -> Value {
    let text = read_to_string("database.json").unwrap();

    from_str::<Value>(&text).unwrap()
}

#[get("/<id>")]
pub fn get_todo(id: usize) -> Either<Value, Status> {
    let text = read_to_string("database.json").unwrap();
    let todos = from_str::<Database>(&text).unwrap().todos;
    if id >= todos.len() {
       return Right(Status::NotFound);
    }

    Left(json!(todos[id]))
}

#[post("/", format="json", data="<todo>")]
pub fn add_todo(todo: Json<NewTodo>) -> Status {
    let text = read_to_string("database.json").unwrap();
    let database = from_str::<Database>(&text).unwrap();
    let mut todos = database.todos;
    let title = todo.into_inner().title;
    todos.push(Todo::new(title));

    let path = "database.json";
    let json_text = to_string::<Database>(&Database { todos }).unwrap();
    std::fs::write(path, &*json_text).unwrap();

    Status::Created
}

#[put("/<id>", format="json", data="<todo>")]
pub fn update_todo(id: usize, todo: Json<UpdateTodo>) -> Status {
    let text = read_to_string("database.json").unwrap();
    let database = from_str::<Database>(&text).unwrap();
    let mut todos = database.todos;
    if id >= todos.len() {
        return Status::NotFound;
    }

    let update_todo = todo.into_inner();
    if update_todo.title.is_some() {
        todos[id].title = update_todo.title.unwrap();
    }

    if update_todo.done.is_some() {
        todos[id].done  = update_todo.done.unwrap();
    }

    let path = "database.json";
    let json_text = to_string::<Database>(&Database { todos }).unwrap();
    std::fs::write(path, &*json_text).unwrap();

    Status::Ok
}
