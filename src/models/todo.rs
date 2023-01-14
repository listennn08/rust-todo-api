use diesel::preload::*;

#[derive(Queryable)]
pub struct Todo {
    pub id:         i32,
    pub title:      String,
    pub done:       bool,
    pub created_at: i64,
}

