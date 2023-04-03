use rocket::{
    http::Status,
    request::{Outcome, Request, FromRequest},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};

use crate::{utils::token::Token, models::user::User};

#[derive(Debug)]
pub struct AuthMiddleware(pub User);

const TOKEN_PREFIX: &str = "Bearer";

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthMiddleware {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        return match token {
            Some(token) => {
                let token = token.to_string().replace(TOKEN_PREFIX, "").trim().to_string();
                return match Token::decode(token) {
                    Ok(user_info) => {
                        let auth_middleware = AuthMiddleware(user_info.clone());
                        request.local_cache(|| { auth_middleware });
                        Outcome::Success(AuthMiddleware(user_info.clone()))
                    },
                    Err(_) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for AuthMiddleware {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let token_schema = SecurityScheme {
            description: Some("Require a token to access".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        let mut token_req = SecurityRequirement::new();
        token_req.insert("Authorization".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "Authorization".to_owned(),
            token_schema,
            token_req,
        ))
    }
}
