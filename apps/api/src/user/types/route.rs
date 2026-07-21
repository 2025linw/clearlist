use chrono_tz::Tz;
use serde::Deserialize;
use sqlx::postgres::types::PgInterval;

use super::UserID;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub struct CreateRequest {
    pub id: UserID,

    pub display_name: String,

    pub preferred_timezone: Option<Tz>,
    pub completed_task_retention: Option<CompletedTaskRetention>,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
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

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub enum CompletedTaskRetention {
    EveryHour,
    EveryDay,
}

impl From<CompletedTaskRetention> for PgInterval {
    fn from(value: CompletedTaskRetention) -> Self {
        match value {
            CompletedTaskRetention::EveryHour => Self {
                months: 0,
                days: 0,
                microseconds: 3_600_000_000,
            },
            CompletedTaskRetention::EveryDay => Self {
                months: 0,
                days: 1,
                microseconds: 0,
            },
        }
    }
}

impl TryFrom<PgInterval> for CompletedTaskRetention {
    type Error = IntervalParseError;

    fn try_from(value: PgInterval) -> Result<Self, Self::Error> {
        match value {
            PgInterval {
                months: 0,
                days: 1,
                microseconds: 0,
            } => Ok(Self::EveryDay),
            PgInterval {
                months: 0,
                days: 0,
                microseconds: 3_600_000_000,
            } => Ok(Self::EveryHour),
            _ => Err(Self::Error {
                reason: format!("{:?}", value),
            }),
        }
    }
}

#[derive(Debug)]
pub struct IntervalParseError {
    reason: String,
}

impl std::error::Error for IntervalParseError {}

impl std::fmt::Display for IntervalParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to parse interval: {}", self.reason)
    }
}
