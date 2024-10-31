use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use serde::Serialize;

use std::fmt::{self, Display};

#[derive(Debug)]
pub enum EzyTutorError {
    DbError(sqlx::Error),
    ActixError(actix_web::Error),
    NotFound(String),
    InvalidInput(String),
}

impl Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbError(_) => write!(f, "Database error"),
            Self::ActixError(_) => write!(f, "Internal server error"),
            Self::NotFound(err) => write!(f, "{}", err),
            Self::InvalidInput(err) => write!(f, "{}", err),
        }
    }
}

impl From<sqlx::Error> for EzyTutorError {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err)
    }
}

impl From<actix_web::Error> for EzyTutorError {
    fn from(err: actix_web::Error) -> Self {
        Self::ActixError(err)
    }
}

impl ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DbError(_) | Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: Self::error_response(self),
        })
    }
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            Self::DbError(err) => eprintln!("Database error occurred: {}", err),
            Self::ActixError(err) => eprintln!("Server error occurred: {}", err),
            Self::NotFound(err) => eprintln!("Not found error occurred: {}", err),
            Self::InvalidInput(err) => eprintln!("Invalid parameters received: {}", err),
        }

        format!("{}", self)
    }
}

#[derive(Debug, Serialize)]
struct MyErrorResponse {
    error_message: String,
}
