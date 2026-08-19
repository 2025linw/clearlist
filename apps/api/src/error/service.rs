use super::{
    Resource,
    repo::{ConstraintViolation, Error as RepoError},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub enum Error {
    Internal(String),

    // Resource Errors
    NotFound(Resource),
    Conflict,
    Validation(ValidationError),

    // Authorization Errors
    Unauthorized,
    Forbidden,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "service error: {:?}", self)
    }
}

impl From<RepoError> for Error {
    fn from(value: RepoError) -> Self {
        match value {
            RepoError::Backend(_) | RepoError::Internal(_) => Self::Internal(value.to_string()),
            RepoError::Constraint(constraint) => match constraint {
                ConstraintViolation::ForeignKey { resource, message } => {
                    if message.contains("created_by") {
                        Self::Internal("expected user record to exist".to_string())
                    } else {
                        Self::NotFound(resource)
                    }
                }
                ConstraintViolation::Unique { .. } => Self::Conflict,
                ConstraintViolation::Check { resource, message } => {
                    unimplemented!("nothing uses check right now: {resource}, {message}")
                }
                ConstraintViolation::SoftDeleted(resource) => Self::NotFound(resource.clone()),
                ConstraintViolation::NotFound(resource) => Self::NotFound(resource.clone()),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    NoChanges,
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },
}

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        Self::Validation(value)
    }
}

// Pagination Error Reasons
pub const NO_ZERO_LIMIT: &str = "limit must be greater than 0";
pub const NO_ZERO_PAGE: &str = "page must be greater than 0";

// Date Filter Error Reasons
pub const RANGE_OVERSPECIFIED: &str = "date range is overspecified";

// Text Error Reasons
pub const NO_NONSPACE_WHITESPACE: &str = "must not contain non-space whitespace characters";
pub const NO_NONMULTILINE_WHITESPACE: &str = "must not contain non-multiline whitespace characters";
pub const NO_EMPTY_STRING: &str = "must not be empty string";
pub const TOO_LONG: &str = "must not exceed max length";
