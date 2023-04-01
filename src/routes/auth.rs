use bcrypt::{hash, DEFAULT_COST, verify};
use either::Either;
use diesel::{
    QueryDsl,
    RunQueryDsl,
    expression_methods::ExpressionMethods,
    prelude::*,
    dsl::count,
};
use rocket::{
    fairing::AdHoc,
    http::Status,
    response::status::Custom,
    serde::{
        Deserialize,
        json::{Json, Value, to_value, json},
    },
};

use crate::db::establish_connection;
use crate::models::error::ErrorResponse;
use crate::models::user::NewUser;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm<'r> {
    pub user_name: &'r str,
    pub password:  &'r str,
}

#[post("/login", data = "<login_form>")]
pub fn login(login_form: Json<LoginForm<'_>>) -> Either<Value, Status> {
    use crate::schema::users::dsl::*;
    use crate::utils::token::Token;

    let login_form = login_form.into_inner();

    let conn = &mut establish_connection();
    let user = users
        .select((user_name, password))
        .filter(user_name.eq(login_form.user_name))
        .first::<(String, String)>(conn)
        .optional()
        .expect("Error: get user failure");

    if let Some(user) = user {
        let (name, user_password) = user;
        let is_verify = verify(login_form.password, &user_password).unwrap();
        if is_verify {
            let token = Token::new(name);
            return Either::Left(json!(token));
        }
    }

    return Either::Right(Status::BadRequest);
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<NewUser<'_>>) -> Either<Status, Custom<Value>> {
    use crate::schema::users::dsl::*;
    use crate::schema::users;

    let pass_user = user.into_inner();

    let conn = &mut establish_connection();
    let user_count = users
        .select(count(user_name))
        .filter(user_name.eq(pass_user.user_name))
        .get_result::<i64>(conn)
        .expect("Error: can't get count from db");

    if user_count > 0 {
        let response = ErrorResponse::new("Username already exist");
        return Either::Right(Custom(
            Status::BadRequest,
            to_value(&response).unwrap()
        ))
    }

    let hashed_password = hash(pass_user.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        user_name: pass_user.user_name,
        password: &hashed_password,
        email: pass_user.email,
        salt_version: Some(1),
    };

    diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error: can't save new user");

    return Either::Left(Status::Created)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth", |rocket| async {
        rocket
            .mount("/api", routes![
                login,
                register,
            ])
    })
}
