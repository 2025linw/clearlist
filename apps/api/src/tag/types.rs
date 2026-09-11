pub mod repo;
pub mod route;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    category::types::{Category, CategoryID},
    user::types::UserID,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type, TS,
)]
#[sqlx(transparent)]
#[ts(export, export_to = "tag/TagID.ts")]
pub struct TagID(Uuid);

impl TagID {
    pub fn new_random() -> Self {
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
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Default))]
pub struct TagModel {
    pub id: TagID,

    pub label: String,
    pub category_id: Option<CategoryID>,
    pub category_name: Option<String>,

    pub position_key: String,
    pub category_position_key: Option<String>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "tag/Tag.ts")]
pub struct Tag {
    pub id: TagID,

    pub label: String,
    pub category: Option<Category>,

    pub position_key: String,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}

impl From<TagModel> for Tag {
    fn from(value: TagModel) -> Self {
        let category = if let Some(id) = value.category_id {
            Some(Category {
                id,
                name: value.category_name.unwrap(),
                position_key: value.category_position_key.unwrap(),
            })
        } else {
            None
        };

        Self {
            id: value.id,
            label: value.label,
            category,
            position_key: value.position_key,
            updated_at: value.updated_at,
            created_at: value.created_at,
            created_by: value.created_by,
        }
    }
}
