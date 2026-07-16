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
            Self::Backend(msg) => write!(f, "backend error: {msg}"),
            Self::Constraint(violation) => write!(f, "constraint violation: {violation}"),
            Self::Programming(msg) => write!(f, "programming error: {msg}"),
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
pub enum ConstraintViolation {
    Generic(String),
    NotFound(Resource),

    MissingUser,

    OwnershipMismatch(Option<String>),

    Integrity(String),

    Unknown(String),
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(msg) => write!(f, "{msg}"),
            Self::NotFound(resource) => write!(f, "{resource} not found"),
            Self::MissingUser => write!(f, "user is missing"),
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
