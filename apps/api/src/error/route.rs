use axum::{http::StatusCode, response::IntoResponse};

use crate::types::response::Response;

use super::{
    Resource,
    service::{Error as ServiceError, ValidationError},
};

#[derive(Debug, Clone)]
pub enum Error {
    Ok,                          // 200
    Created,                     // 201
    NoContent,                   // 204
    BadRequest(String),          // 400
    Unauthenticated,             // 401
    NotAuthorized,               // 403
    NotFound(Resource),          // 404
    Conflict,                    // 409
    InternalServerError(String), // 500
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "route error: {:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::new(StatusCode::from(self.clone()))
            .message(self.to_string())
            .into_response()
    }
}

impl From<ServiceError> for Error {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Internal(error) => Self::InternalServerError(error.to_string()),
            ServiceError::NotFound(resource) => Self::NotFound(resource),
            ServiceError::Conflict => Self::Conflict,
            ServiceError::Validation(validation) => match validation {
                ValidationError::NoChanges => Self::BadRequest("no changes requested".to_string()),
                ValidationError::InvalidValue { field, reason } => {
                    Self::BadRequest(format!("invalid value found for '{field}': {reason}"))
                } // ValidationError::Unique => Self::Conflict,
            },
            ServiceError::Unauthorized => Self::Unauthenticated,
            ServiceError::Forbidden => Self::NotAuthorized,
        }
    }
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Ok => StatusCode::OK,
            Error::Created => StatusCode::CREATED,
            Error::NoContent => StatusCode::NO_CONTENT,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthenticated => StatusCode::UNAUTHORIZED,
            Error::NotAuthorized => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Conflict => StatusCode::CONFLICT,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
