use serde::Deserialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Type)]
#[sqlx(transparent)]
pub struct UserID(pub Uuid);

impl UserID {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct Model {
    pub id: UserID,

    pub display_name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}
