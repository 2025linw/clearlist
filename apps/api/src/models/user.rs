use chrono::Utc;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Model {
    pub id: uuid::Uuid,

    pub display_name: String,

    pub created_at: chrono::DateTime<Utc>,
}
