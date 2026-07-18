pub mod repo;
pub mod route;

use serde::Deserialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::user::types::UserID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TagID(Uuid);

impl TagID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TagID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Deserialize)]
pub enum SortBy {
    ID,
    Created,
    Updated,
    Position,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::ID => write!(f, "id"),
            SortBy::Created => write!(f, "created_at"),
            SortBy::Updated => write!(f, "updated_at"),
            SortBy::Position => write!(f, "position_key"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Clone))]
pub struct Model {
    pub id: TagID,

    pub label: String,
    pub category: Option<String>,

    pub position_key: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub created_by: UserID,
}
