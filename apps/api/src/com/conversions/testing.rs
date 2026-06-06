//! Testing Conversions
//!
//! This module contains type conversions only used in tests
//!
//! They are not supported or useful operations in normal operation
//!
//! Conversions:
//! * from model-level Task to user request Task
//! * From model-level Tag to user request Tag

use crate::models::{
    helper::Start,
    tag::{DtoModel as TagCreate, Model as TagModel},
    task::{DtoModel as TaskCreate, Model as TaskModel},
};

impl From<TaskModel> for TaskCreate {
    fn from(value: TaskModel) -> Self {
        let start = if let Some(dt) = value.start_dt {
            if value.has_time {
                Some(Start::At(dt))
            } else {
                Some(Start::On(dt.date_naive()))
            }
        } else {
            None
        };

        Self {
            title: value.title,
            notes: value.notes,
            start,
            deadline: value.deadline,
            tags: value.tags.iter().map(|tag| tag.id).collect(),
        }
    }
}

impl From<TagModel> for TagCreate {
    fn from(value: TagModel) -> Self {
        Self {
            label: value.label,
            category: value.category,
        }
    }
}
