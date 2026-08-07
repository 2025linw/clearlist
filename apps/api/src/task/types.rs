pub mod repo;
pub mod route;
pub mod service;

use chrono_tz::Tz;
use serde::Deserialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::{
    tag::types::{Tag, TagModel},
    types::date::Start,
    user::types::UserID,
};

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
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
pub struct TaskModel {
    pub id: TaskID,

    pub title: String,
    pub notes: Option<String>,
    pub start_dt: Option<chrono::DateTime<chrono::Utc>>,
    pub has_time: bool,
    pub deadline: Option<chrono::NaiveDate>,

    pub position_key: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub id: TaskID,

    pub title: String,
    pub notes: Option<String>,
    pub start: Option<Start>,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<Tag>,

    pub position_key: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

impl Task {
    pub fn from(value: TaskModel, tz: Tz, tags: Vec<TagModel>) -> Self {
        let start = value.start_dt.map(|start_dt| {
            if value.has_time {
                Start::DateTime(start_dt)
            } else {
                Start::Date(start_dt.with_timezone(&tz).date_naive())
            }
        });

        Self {
            id: value.id,
            title: value.title,
            notes: value.notes,
            start,
            deadline: value.deadline,
            tags: tags.into_iter().map(Tag::from).collect(),
            position_key: value.position_key,
            completed_at: value.completed_at,
            deleted_at: value.deleted_at,
            updated_at: value.updated_at,
            created_at: value.created_at,
            created_by: value.created_by,
        }
    }
}
