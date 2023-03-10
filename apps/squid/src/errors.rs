// See: https://github.com/clifinger/canduma/blob/master/src/errors.rs
use actix_web::HttpResponse;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Clone)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("{0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Database Error")]
    InternalDatabaseError,
}

impl From<sqlx::Error> for ServiceError {
    fn from(_value: sqlx::Error) -> Self {
        ServiceError::InternalDatabaseError
    }
}

impl actix_web::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
            ServiceError::InternalServerError => HttpResponse::InternalServerError().finish(),
            ServiceError::InternalDatabaseError => {
                HttpResponse::InternalServerError().body("Internal Database Error")
            }
            ServiceError::BadRequest(s) => HttpResponse::BadRequest().body(s.to_string()),
        }
    }
}

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
