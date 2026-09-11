use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgInterval;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletedTaskRetention {
    #[serde(alias = "hour")]
    EveryHour,
    #[serde(alias = "day")]
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
