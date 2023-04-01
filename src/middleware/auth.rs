use rocket::{
    http::Status,
    request::{Outcome, Request, FromRequest},
};
use crate::{utils::token::Token, models::user::User};

#[derive(Debug)]
pub struct AuthMiddleware(pub User);

const TOKEN_PREFIX: &str = "Bearer";

#[rocket::async_trait]
impl<'a> FromRequest<'a> for &'a AuthMiddleware {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        return match token {
            Some(token) => {
                let token = token.to_string().replace(TOKEN_PREFIX, "").trim().to_string();
                return match Token::decode(token) {
                    Ok(user_info) => {
                        Outcome::Success(request.local_cache(|| {
                            AuthMiddleware(user_info)
                        }))
                    },
                    Err(_) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

