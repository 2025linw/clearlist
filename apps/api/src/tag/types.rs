pub mod repo;
pub mod route;
pub mod service;

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

impl std::fmt::Display for TagID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct CategoryID(Uuid);

impl CategoryID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CategoryID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for CategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct TagModel {
    pub id: TagID,

    pub label: String,
    pub category_id: Option<CategoryID>,
    pub category_name: Option<String>,

    pub position_key: String,
    pub cat_position_key: Option<String>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    pub id: TagID,

    pub label: String,
    pub category_id: Option<CategoryID>,
    pub category_name: Option<String>,

    pub position_key: String,
    pub cat_position_key: Option<String>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

impl From<TagModel> for Tag {
    fn from(value: TagModel) -> Self {
        Self {
            id: value.id,
            label: value.label,
            category_id: value.category_id,
            category_name: value.category_name,
            position_key: value.position_key,
            cat_position_key: value.cat_position_key,
            updated_at: value.updated_at,
            created_at: value.created_at,
            created_by: value.created_by,
        }
    }
}
