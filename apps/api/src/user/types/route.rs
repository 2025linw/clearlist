use chrono_tz::Tz;
use serde::Deserialize;

use crate::types::field::CompletedTaskRetention;

use super::UserID;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionRequest {
    pub id: UserID,

    pub display_name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub display_name: Option<String>,

    pub preferred_timezone: Option<Option<Tz>>,
    pub completed_task_retention: Option<Option<CompletedTaskRetention>>,
}

impl UpdateRequest {
    pub fn is_noop(&self) -> bool {
        let Self {
            display_name,
            preferred_timezone,
            completed_task_retention,
        } = self;

        display_name.is_none() && preferred_timezone.is_none() && completed_task_retention.is_none()
    }
}
