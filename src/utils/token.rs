use chrono::{Utc, Duration};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::Error};
use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::db::establish_connection;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Token {
    token: String,
}

const SECRET_KEY: &str = "SECRET_KEY";

impl Token {
    pub fn new(user_name: String) -> Token {
        let encoding_key = EncodingKey::from_secret(SECRET_KEY.as_ref());
        let current_time = Utc::now();
        let expire_time = current_time + Duration::days(7);
        let claims = Claims {
            sub: user_name.to_owned(),
            exp: expire_time.timestamp(),
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &encoding_key,
        ).unwrap();

        Token {
            token
        }
    }

    pub fn decode<'r>(token: String) -> Result<User, Error>{
        use crate::schema::users::dsl::*;

        let conn = &mut establish_connection();
        let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_ref());
        let decode_token = decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::HS256));
        
        return match decode_token {
            Ok(info) => {
                let user = users
                    .filter(user_name.eq(info.claims.sub))
                    .first::<User>(conn)
                    .expect("Can't get user");
                return Ok(user);
            },
            Err(err) => Result::Err(err)
        }
    }
}
