pub mod repo;
pub mod route;

use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use ts_rs::TS;
use uuid::Uuid;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type, TS
)]
#[sqlx(transparent)]
#[ts(export, export_to = "category/CategoryID.ts")]
pub struct CategoryID(Uuid);

impl CategoryID {
    pub fn new_random() -> Self {
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
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Default))]
pub struct CategoryModel {
    pub id: CategoryID,
    pub name: String,
    pub position_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct Category {
    pub id: CategoryID,
    pub name: String,
    pub position_key: String,
}

impl From<CategoryModel> for Category {
    fn from(value: CategoryModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            position_key: value.position_key,
        }
    }
}
