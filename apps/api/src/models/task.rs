//! Task Model
//!
//! This module contains the task model

use chrono::Utc;
use serde::Deserialize;
use sqlx::FromRow;
use ts_rs::TS;

use super::{helper::Start, tag::Model as TagModel};

/// Task Model Type
///
/// This is the ground truth model for tasks; it exactly matches database schema
#[allow(dead_code)]
#[derive(Debug, FromRow, TS)]
#[cfg_attr(test, derive(Clone, PartialEq))]
#[ts(export, rename = "Task")]
pub struct Model {
    pub id: uuid::Uuid,

    pub title: String,
    pub notes: Option<String>,
    pub start_dt: Option<chrono::DateTime<Utc>>,
    pub has_time: bool,
    pub deadline: Option<chrono::NaiveDate>,
    #[sqlx(skip)]
    pub tags: Vec<TagModel>,

    pub completed_at: Option<chrono::DateTime<Utc>>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub created_by: uuid::Uuid,
}

/// Task DTO Model
///
/// This represents the fields a client is able to create/modify for a Task
#[derive(Debug, Deserialize, TS)]
#[cfg_attr(test, derive(Default, Clone))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[ts(export, rename = "TaskDTO")]
pub struct DtoModel {
    #[serde(default)]
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<Start>,
    pub deadline: Option<chrono::NaiveDate>,
    #[serde(default)]
    pub tags: Vec<uuid::Uuid>,
}

/// Task Tag Type
///
/// This is an intermediate model for task tags returned by the database
///
/// This is currently only used in querying tasks
#[derive(Debug, FromRow)]
pub struct TaskTag {
    pub task_id: uuid::Uuid,

    #[sqlx(flatten)]
    pub tag: TagModel,
}
