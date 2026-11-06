use chrono_tz::Tz;
use serde::Deserialize;

use crate::types::{field::CompletedTaskRetention, route::UpdateField};

use super::UserID;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionRequest {
    pub id: UserID,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub preferred_timezone: UpdateField<Tz>,
    pub completed_task_retention: UpdateField<CompletedTaskRetention>,
}

impl UpdateRequest {
    pub fn is_noop(&self) -> bool {
        let Self {
            preferred_timezone,
            completed_task_retention,
        } = self;

        preferred_timezone.is_none() && completed_task_retention.is_none()
    }
}
