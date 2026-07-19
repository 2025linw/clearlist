use super::Resource;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub enum Error {
    Backend(String),

    Constraint(ConstraintViolation),

    Programming(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(msg) => write!(f, "repo: backend error - {msg}"),
            Self::Constraint(violation) => write!(f, "repo: onstraint violation - {violation}"),
            Self::Programming(msg) => write!(f, "repo: programming error - {msg}"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            // Backend
            sqlx::Error::Configuration(_)
            | sqlx::Error::Io(_)
            | sqlx::Error::Tls(_)
            | sqlx::Error::Protocol(_)
            | sqlx::Error::AnyDriverError(_)
            | sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::WorkerCrashed
            | sqlx::Error::InvalidSavePointStatement
            | sqlx::Error::BeginFailed => Self::Backend(value.to_string()),

            // Programming
            err => Self::Programming(err.to_string()),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub enum ConstraintViolation {
    NotFound(Resource),
    MissingUser,
}

impl ConstraintViolation {}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(resource) => write!(f, "{resource} not found"),
            Self::MissingUser => write!(f, "user is missing"),
        }
    }
}
