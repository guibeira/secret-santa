use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error, PartialEq)]
pub enum CustomError {
    #[display(fmt = "{}", error)]
    ValidationError { error: String },
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&ErrorResponse {
            error: self.to_string(),
        });

        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(body.unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
