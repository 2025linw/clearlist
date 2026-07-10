use sqlx::postgres::PgDatabaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Backend(String),

    Constraint(ConstraintViolation),

    Programming(String),
    Unknown(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(msg) => write!(f, "backend error: {msg}"),
            Self::Constraint(violation) => write!(f, "constraint violation: {violation}"),
            Self::Programming(msg) => write!(f, "programming error: {msg}"),
            Self::Unknown(msg) => write!(f, "unknown/uncaught error: {msg}"),
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
            | sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::WorkerCrashed
            | sqlx::Error::BeginFailed
            | sqlx::Error::InvalidSavePointStatement => Self::Backend(value.to_string()),

            // Programming
            sqlx::Error::InvalidArgument(_)
            | sqlx::Error::RowNotFound
            | sqlx::Error::TypeNotFound { .. }
            | sqlx::Error::ColumnIndexOutOfBounds { .. }
            | sqlx::Error::ColumnNotFound(_)
            | sqlx::Error::ColumnDecode { .. }
            | sqlx::Error::Encode(_)
            | sqlx::Error::Decode(_)
            | sqlx::Error::Migrate(_) => Self::Programming(value.to_string()),

            // Database error
            sqlx::Error::Database(err) => {
                if let Some(pg_err) = err.try_downcast_ref::<PgDatabaseError>() {
                    let code = pg_err.code();
                    let message = pg_err.message();
                    if code == "2201W" {
                        // Invalid row count in limit

                        Self::Programming(format!("invalid LIMIT found: {}", pg_err.message()))
                    } else if code == "2201X" {
                        // Invalid row count in result offset

                        Self::Programming(format!("invalid OFFSET found: {}", pg_err.message()))
                    } else if code.starts_with("23") {
                        // Integrity Constraint

                        Self::Constraint(ConstraintViolation::Integrity(
                            pg_err.message().to_string(),
                        ))
                    } else if code == "P0001" {
                        if message == "resource_not_found" {
                            if let Some(detail) = pg_err.detail() {
                                if detail.contains("task") {
                                    Self::Constraint(ConstraintViolation::NotFound(Resource::Task))
                                } else if detail.contains("tag") {
                                    Self::Constraint(ConstraintViolation::NotFound(Resource::Tag))
                                } else {
                                    Self::Constraint(ConstraintViolation::Unknown(format!(
                                        "unhandled resource_not_found - {detail}"
                                    )))
                                }
                            } else {
                                Self::Constraint(ConstraintViolation::Unknown(
                                    "unhandled resource_not_found".to_string(),
                                ))
                            }
                        } else if message == "resource_deleted" {
                            if let Some(detail) = pg_err.detail() {
                                if detail.contains("task") {
                                    Self::Constraint(ConstraintViolation::IsDeleted(Resource::Task))
                                } else {
                                    Self::Constraint(ConstraintViolation::Unknown(format!(
                                        "unhandled resource_deleted - {detail}"
                                    )))
                                }
                            } else {
                                Self::Constraint(ConstraintViolation::Unknown(
                                    "unhandled resource_deleted".to_string(),
                                ))
                            }
                        } else if message == "ownership_mismatch" {
                            if let Some(detail) = pg_err.detail() {
                                Self::Constraint(ConstraintViolation::OwnershipMismatch(Some(
                                    detail.to_string(),
                                )))
                            } else {
                                Self::Constraint(ConstraintViolation::OwnershipMismatch(None))
                            }
                        } else {
                            Self::Unknown(pg_err.to_string())
                        }
                    } else {
                        Self::Unknown(pg_err.to_string())
                    }
                } else {
                    Self::Unknown(err.to_string())
                }
            }
            err => Self::Unknown(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum ConstraintViolation {
    Generic(String),

    NotFound(Resource),
    IsDeleted(Resource),
    OwnershipMismatch(Option<String>),

    Integrity(String),

    Unknown(String),
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(msg) => write!(f, "{msg}"),
            Self::NotFound(resource) => write!(f, "{resource} not found"),
            Self::IsDeleted(resource) => write!(f, "{resource} is deleted"),
            Self::OwnershipMismatch(opt) => {
                if let Some(msg) = opt {
                    write!(f, "ownership does not match - {msg}")
                } else {
                    write!(f, "ownership does not match")
                }
            }
            Self::Integrity(msg) => write!(f, "{msg}"),
            Self::Unknown(msg) => write!(f, "unknown - {msg}"),
        }
    }
}

#[derive(Debug)]
pub enum Resource {
    User,
    Task,
    Tag,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::User => write!(f, "user"),
            Resource::Task => write!(f, "task"),
            Resource::Tag => write!(f, "tag"),
        }
    }
}
