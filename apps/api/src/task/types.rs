pub mod repo;
pub mod route;

use serde::Deserialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::{tag::types::Model as TagModel, user::types::UserID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TaskID(Uuid);

impl TaskID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Deserialize)]
pub enum SortBy {
    ID,
    Created,
    Updated,
    Start,
    Deadline,
    Position,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::ID => write!(f, "id"),
            SortBy::Created => write!(f, "created_at"),
            SortBy::Updated => write!(f, "updated_at"),
            SortBy::Start => write!(f, "start_dt"),
            SortBy::Deadline => write!(f, "deadline"),
            SortBy::Position => write!(f, "position_key"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Clone))]
pub struct Model {
    pub id: TaskID,

    pub title: String,
    pub notes: Option<String>,
    pub start_dt: Option<chrono::DateTime<chrono::Utc>>,
    pub has_time: bool,
    pub deadline: Option<chrono::NaiveDate>,
    #[sqlx(skip)]
    pub tags: Vec<TagModel>,

    pub position_key: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub created_by: UserID,
}
