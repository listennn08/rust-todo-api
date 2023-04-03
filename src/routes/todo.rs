use either::Either;
use rocket::{
    serde::json::Json,
    http::Status,
};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, JsonSchema, openapi_get_routes_spec};
use serde::Deserialize;
use diesel::{
    QueryDsl,
    RunQueryDsl,
    expression_methods::ExpressionMethods, result::Error::NotFound,
};
use chrono::Utc;
use std::vec::Vec;

use crate::db::establish_connection;
use crate::models::todo::{Todo, NewTodo, UpdateTodo};
use crate::middleware::auth::AuthMiddleware;

#[openapi(tag = "Todo")]
#[get("/", format = "json")]
async fn get_all_todos(auth: AuthMiddleware) -> Json<Vec<Todo>> {
    use crate::schema::todos::dsl::*;

    // get user info from middleware
    let user_info = auth.0.clone();

    let conn = &mut establish_connection();
    let mut query = todos.into_boxed();

    if user_info.role != "admin" {
        query = query.filter(created_by.eq(user_info.id));
    }

    let results = query.load::<Todo>(conn).expect("Error: loading todos failure");
    
    Json(results)
    // to_value(&results).unwrap()
}

#[openapi(tag = "Todo")]
#[get("/<todo_id>", format = "json")]
fn get_todo(auth: AuthMiddleware, todo_id: i32) -> Either<Json<Todo>, Status> {
    use crate::schema::todos::dsl::*;

    let user_info = auth.0.clone();

    let conn = &mut establish_connection();
    let mut query = todos.into_boxed().filter(id.eq(todo_id));

    if user_info.role != "admin" {
       query = query.filter(created_by.eq(user_info.id));
    }

    let result = query.first::<Todo>(conn);

    match result {
        Ok(result) => Either::Left(Json(result)),
        Err(NotFound) => Either::Right(Status::NotFound),
        Err(_) => {
            println!("Error: loading todo item failure");
            Either::Right(Status::ServiceUnavailable)
        },
    }
}

#[derive(Deserialize, JsonSchema)]
struct TodoPayload {
    title: String
}

#[openapi(tag = "Todo")]
#[post("/", format="json", data="<todo>")]
fn add_todo(auth: AuthMiddleware, todo: Json<TodoPayload>) -> Status {
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

#[openapi(tag = "Todo")]
#[put("/<todo_id>", format="json", data="<todo>")]
fn update_todo(auth: AuthMiddleware,todo_id: i32, todo: Json<UpdateTodo>) -> Status {
    use crate::schema::todos::dsl::*;

    let user_info = auth.0.clone();

    let conn = &mut establish_connection();
    let mut query = diesel::update(todos).into_boxed();

    if user_info.role != "admin" {
       query = query.filter(created_by.eq(user_info.id));
    }

    query.filter(id.eq(todo_id))
        .set((
            title.eq(todo.title),
            done.eq(todo.done),
            updated_at.eq(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        ))
        .execute(conn)
        .expect("Update todo error");

    Status::Ok
}

#[openapi(tag = "Todo")]
#[delete("/<todo_id>")]
fn delete_todo(auth: AuthMiddleware, todo_id: i32) -> Status {
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

pub fn get_routes_and_spec() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        get_all_todos,
        get_todo,
        add_todo,
        update_todo,
        delete_todo,
    ]
}

