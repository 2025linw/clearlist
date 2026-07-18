use sqlx::postgres::PgDatabaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
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

            // Database
            sqlx::Error::Database(err) => {
                let mut error = Self::Programming(err.to_string());

                if let Some(pg_err) = err.try_downcast_ref::<PgDatabaseError>()
                    && let Some(integrity) = Integrity::try_from_code(pg_err.code())
                {
                    error = Self::Constraint(ConstraintViolation::Integrity(integrity));
                }

                error
            }

            // Programming
            err => Self::Programming(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum ConstraintViolation {
    NotFound(Resource),
    MissingUser,

    Integrity(Integrity),

    Unknown(String),
}

impl ConstraintViolation {}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(resource) => write!(f, "{resource} not found"),
            Self::MissingUser => write!(f, "user is missing"),
            Self::Integrity(msg) => write!(f, "{msg}"),
            Self::Unknown(msg) => write!(f, "unknown - {msg}"),
        }
    }
}

#[derive(Debug)]
pub enum Integrity {
    NotNull,    // 23502
    ForeignKey, // 23503
    Unique,     // 23505
    Check,      // 23514
}

impl Integrity {
    pub fn try_from_code(code: &str) -> Option<Self> {
        match code {
            "23502" => Some(Self::NotNull),
            "23503" => Some(Self::ForeignKey),
            "23505" => Some(Self::Unique),
            "23514" => Some(Self::Check),
            _ => None,
        }
    }
}

impl std::fmt::Display for Integrity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integrity::NotNull => write!(f, "not null"),
            Integrity::ForeignKey => write!(f, "foreign key"),
            Integrity::Unique => write!(f, "unique"),
            Integrity::Check => write!(f, "check"),
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
