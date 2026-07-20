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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TagCategoryID(Uuid);

impl TagCategoryID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TagCategoryID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct TagModel {
    pub id: TagID,

    pub label: String,
    pub category_id: Option<TagCategoryID>,
    pub category_name: Option<String>,

    pub position_key: String,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: UserID,
}
