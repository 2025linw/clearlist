#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use super::{
    Resource,
    repo::{ConstraintViolation, Error as RepoError},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub enum Error {
    NotFound(Resource),
    Deleted(Resource),

    Validation(ValidationError),

    Forbidden,
    Conflict,
    Unauthorized,

    Internal(RepoError),
    Unhandled(RepoError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "service error: {:?}", self)
    }
}

impl From<RepoError> for Error {
    fn from(value: RepoError) -> Self {
        match &value {
            RepoError::Backend(_) => Self::Internal(value),
            RepoError::Programming(_) => Self::Unhandled(value),
            RepoError::Constraint(constraint) => match constraint {
                ConstraintViolation::NotFound(resource) => Self::NotFound(resource.clone()),
                ConstraintViolation::Deleted(resource) => Self::Deleted(resource.clone()),
                ConstraintViolation::Unique(resource) => Self::Validation(ValidationError::Unique),
                ConstraintViolation::MissingUser => Self::Internal(value),
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
    Unique,
}

// Pagination Error Reasons
pub const ZERO_LIMIT_REASON: &str = "limit must be greater than 0";
pub const ZERO_PAGE_REASON: &str = "page must be greater than 0";

// Text Error Reasons
pub const NO_WHITESPACE_REASON: &str = "must not contain non-space whitespace characters";
pub const NO_EMPTY_STRING: &str = "must not be empty string";
