use chrono::Utc;
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Debug, FromRow, TS)]
#[ts(export, rename = "Task")]
pub struct Model {
    pub id: uuid::Uuid,

    pub display_name: String,

    pub created_at: chrono::DateTime<Utc>,
}
