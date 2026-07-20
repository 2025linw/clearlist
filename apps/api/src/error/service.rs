#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use super::{Resource, repo::Error as RepoError};

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
        match value {
            RepoError::Backend(_) => Self::Internal(value),
            RepoError::Programming(_) | RepoError::Constraint(_) => Self::Unhandled(value),
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

// Pagination Error Reasons
const ZERO_LIMIT_REASON: &str = "limit must be greater than 0";
const ZERO_PAGE_REASON: &str = "page must be greater than 0";

// Text Error Reasons
const NO_WHITESPACE_REASON: &str = "contains whitespace characters: [\\t, \\n]";
const NO_EMPTY_CATEGORY_REASON: &str = "category must not be empty string";
