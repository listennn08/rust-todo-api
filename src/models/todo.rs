use diesel::prelude::*;
use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::schema::todos;

#[derive(Queryable, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id:         i32,
    pub title:      String,
    pub done:       bool,
    pub created_by: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title:      &'a str,
    pub created_by: i32,
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = todos)]
pub struct UpdateTodo<'a> {
    pub title: &'a str,
    pub done: bool,
}
