pub mod repo;
pub mod route;

use serde::Deserialize;
use sqlx::{FromRow, Type, postgres::types::PgInterval};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct UserID(pub Uuid);

impl UserID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
#[cfg_attr(test, derive(Clone))]
pub struct UserModel {
    pub id: UserID,

    pub display_name: String,

    pub preferred_timezone: Option<String>,
    pub completed_task_retention: Option<PgInterval>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
