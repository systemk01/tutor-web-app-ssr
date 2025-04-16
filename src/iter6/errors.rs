use actix_web::{HttpResponse, error, http::StatusCode};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for EzyTutorError {}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occured: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occured: {:?}", msg);
                msg.into()
            }
            EzyTutorError::TeraError(msg) => {
                println!("Invalid input received {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(msg)
            | EzyTutorError::ActixError(msg)
            | EzyTutorError::TeraError(msg) => {
                println!("Internal Server Error Statuscode: 500: {:?}", msg);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error, Statuscode: 400: {:?}", msg);
                StatusCode::NOT_FOUND
            }
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}
