use rocket_okapi::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct ErrorResponse {
    message: String
}

impl ErrorResponse {
    pub fn new(message: &str) -> ErrorResponse {
        ErrorResponse {
            message: message.to_string()
        }
    }
}
