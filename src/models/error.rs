use serde::Serialize;

#[derive(Serialize)]
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
