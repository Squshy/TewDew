// See: https://github.com/clifinger/canduma/blob/master/src/errors.rs
use juniper::graphql_value;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Service Error")]
    InternalServerError,

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Database Error")]
    InternalDatabaseError,
}

impl juniper::IntoFieldError for ServiceError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ServiceError::InternalServerError => juniper::FieldError::new(
                "Internal Server Error",
                graphql_value!({
                    "type": "INTERNAL_SERVICE_ERROR"
                }),
            ),
            ServiceError::BadRequest(s) => juniper::FieldError::new(
                s,
                graphql_value!({
                    "type": "BAD_REQUEST",
                }),
            ),
            ServiceError::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "UNAUTHORIZED"
                }),
            ),
            ServiceError::InternalDatabaseError => juniper::FieldError::new(
                "Internal Database Error",
                graphql_value!({
                    "type": "INTERNAL_DATABASE_ERROR"
                }),
            ),
        }
    }
}

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
