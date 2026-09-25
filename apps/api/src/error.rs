pub mod repo;
pub mod route;
pub mod service;

#[derive(Debug, Clone)]
pub enum Resource {
    User,
    Task,
    Tag,
    Category,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::User => write!(f, "user"),
            Resource::Task => write!(f, "task"),
            Resource::Tag => write!(f, "tag"),
            Resource::Category => write!(f, "category"),
        }
    }
}
