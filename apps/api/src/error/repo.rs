use sqlx::{error::DatabaseError, postgres::PgDatabaseError};

use super::Resource;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub enum Error {
    Backend(String),
    Internal(String),

    // Resource Error
    Constraint(ConstraintViolation),
}

impl Error {
    pub fn from_sqlx(value: sqlx::Error, resource: Resource) -> Self {
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

            sqlx::Error::Database(ref db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                if pg_err.is_foreign_key_violation() {
                    // FK
                    Self::Constraint(ConstraintViolation::ForeignKey {
                        resource,
                        message: pg_err.message().to_string(),
                    })
                } else if pg_err.is_unique_violation() {
                    // Unique/PK
                    Self::Constraint(ConstraintViolation::Unique {
                        resource,
                        message: pg_err.message().to_string(),
                    })
                } else if pg_err.is_check_violation() {
                    // Check
                    Self::Constraint(ConstraintViolation::Check {
                        resource,
                        message: pg_err.message().to_string(),
                    })
                } else {
                    let message = pg_err.message();
                    if message.contains("ownership_mismatch")
                        || message.contains("resource_not_found")
                    {
                        Self::Constraint(ConstraintViolation::NotFound(resource))
                    } else {
                        Self::Internal(format!("database: {pg_err}"))
                    }
                }
            }
            sqlx::Error::RowNotFound => Self::Constraint(ConstraintViolation::NotFound(resource)),

            // Internal
            err => Self::Internal(err.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(msg) => write!(f, "repo - backend error: {msg}"),
            Self::Constraint(violation) => write!(f, "repo - constraint violation: {violation}"),
            Self::Internal(msg) => write!(f, "repo - internal error: {msg}"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Configuration(_)
            | sqlx::Error::Io(_)
            | sqlx::Error::Tls(_)
            | sqlx::Error::Protocol(_)
            | sqlx::Error::AnyDriverError(_)
            | sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::WorkerCrashed
            | sqlx::Error::InvalidSavePointStatement
            | sqlx::Error::BeginFailed => Self::Internal(value.to_string()),
            err => Self::Internal(format!("Unhandled error: {err}")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstraintViolation {
    ForeignKey { resource: Resource, message: String },
    Unique { resource: Resource, message: String },
    Check { resource: Resource, message: String },

    SoftDeleted(Resource),
    NotFound(Resource),
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForeignKey { resource, message } => {
                write!(f, "[foreign key] {resource}: {message}")
            }
            Self::Unique { resource, message } => {
                write!(f, "[unique] {resource}: {message}")
            }
            Self::Check { resource, message } => {
                write!(f, "[check] {resource}: {message}")
            }
            Self::SoftDeleted(resource) => write!(f, "[deleted] {resource} is marked deleted"),
            Self::NotFound(resource) => write!(f, "[not found] {resource}: not found"),
        }
    }
}

impl From<ConstraintViolation> for Error {
    fn from(value: ConstraintViolation) -> Self {
        Self::Constraint(value)
    }
}
