pub mod repo;
pub mod route;

use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type, postgres::types::PgInterval};
use uuid::Uuid;

use crate::types::field::CompletedTaskRetention;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type,
)]
#[sqlx(transparent)]
pub struct UserID(pub Uuid);

impl UserID {
    pub fn new_random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct UserModel {
    pub id: UserID,

    pub display_name: String,

    pub preferred_timezone: Option<String>,
    pub completed_task_retention: Option<PgInterval>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: UserID,

    pub display_name: String,

    pub preferred_timezone: Option<Tz>,
    pub completed_task_retention: Option<CompletedTaskRetention>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn from(value: UserModel) -> Self {
        let preferred_timezone = value.preferred_timezone.map(|str| str.parse().unwrap());
        let completed_task_retention = value
            .completed_task_retention
            .map(|interval| CompletedTaskRetention::try_from(interval).unwrap());

        Self {
            id: value.id,
            display_name: value.display_name,
            preferred_timezone,
            completed_task_retention,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
