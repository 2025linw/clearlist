use sqlx::{QueryBuilder, postgres::types::PgInterval};

use super::UserID;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct CreateModel {
    pub id: UserID,

    pub display_name: String,

    pub preferred_timezone: Option<String>,
    pub completed_task_retention: Option<PgInterval>,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub display_name: Option<String>,

    pub preferred_timezone: Option<Option<String>>,
    pub completed_task_retention: Option<Option<PgInterval>>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(display_name) = self.display_name {
            separated.push("display_name = ");
            separated.push_bind_unseparated(display_name);
        }
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
