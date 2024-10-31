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
    TeraError(tera::Error),
}

impl From<actix_web::Error> for EzyTutorError {
    fn from(err: actix_web::Error) -> Self {
        Self::ActixError(err)
    }
}

impl From<sqlx::Error> for EzyTutorError {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err)
    }
}

impl Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbError(_) => write!(f, "Database error"),
            Self::ActixError(_) => write!(f, "Internal server error"),
            Self::TeraError(_) => write!(f, "Template render error"),
            Self::NotFound(err) => write!(f, "{}", err),
        }
    }
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DbError(err) => eprintln!("Database error occurred: {}", err),
            EzyTutorError::ActixError(err) => eprintln!("Server error occurred: {}", err),
            EzyTutorError::TeraError(err) => eprintln!("Error in rendering the template {}", err),
            EzyTutorError::NotFound(err) => eprintln!("Not found error occurred: {:?}", err),
        }
        format!("{}", self)
    }
}

impl std::error::Error for EzyTutorError {}

impl ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DbError(_) | Self::ActixError(_) | Self::TeraError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: Self::error_response(self),
        })
    }
}

#[derive(Debug, Serialize)]
struct MyErrorResponse {
    error_message: String,
}
