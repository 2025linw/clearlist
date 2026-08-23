pub mod repo;
pub mod route;

use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    tag::types::{Tag, TagModel},
    types::start::Start,
    user::types::UserID,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type, TS,
)]
#[sqlx(transparent)]
#[ts(export, export_to = "task/TaskID.ts")]
pub struct TaskID(Uuid);

impl TaskID {
    pub fn new_random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for TaskID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
            SortBy::Start => write!(f, "start"),
            SortBy::Deadline => write!(f, "deadline"),
            SortBy::Position => write!(f, "position_key"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct TaskModel {
    pub id: TaskID,

    pub title: String,
    pub notes: Option<String>,
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub has_time: bool,
    pub deadline: Option<chrono::NaiveDate>,

    pub position_key: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task/Task.ts")]
pub struct Task {
    pub id: TaskID,

    pub title: String,
    pub notes: Option<String>,
    #[ts(type = "string | null")]
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
        let start = value.start.map(|dt| {
            if value.has_time {
                Start::DateTime(dt)
            } else {
                Start::Date(dt.with_timezone(&tz).date_naive())
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
