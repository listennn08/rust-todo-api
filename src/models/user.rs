use diesel::{prelude::*, Queryable};
use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct User {
    pub id:           i32,
    pub user_name:    String,
    pub password:     String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email:        Option<String>,
    pub role:         String,
    pub salt_version: i32,
    pub created_at:   String,
    pub updated_at:   String,
}

#[derive(Insertable, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub email:  Option<String>,
    pub salt_version: Option<i32>,
}
