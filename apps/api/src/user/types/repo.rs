use sqlx::{QueryBuilder, postgres::types::PgInterval};

use super::UserID;

#[derive(Debug, Clone)]
pub struct CreateModel {
    pub id: UserID,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateModel {
    pub preferred_timezone: Option<Option<String>>,
    pub completed_task_retention: Option<Option<PgInterval>>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(timezone_str) = self.preferred_timezone {
            separated.push("preferred_timezone = ");
            separated.push_bind_unseparated(timezone_str);
        }
        if let Some(opt) = self.completed_task_retention {
            separated.push("completed_task_retention = ");
            separated.push_bind_unseparated(opt);
        }
    }
}
