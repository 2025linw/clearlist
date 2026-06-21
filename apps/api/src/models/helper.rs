use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Start Precision Type
///
/// This allows for the Dto's to specify when a timestamp only specifies the date or specifies both date and time
#[derive(Debug, Default, Clone, Deserialize, Serialize, sqlx::Type, TS)]
#[sqlx(type_name = "start_mode", rename_all = "lowercase")]
#[ts(export)]
pub enum TimestampPrecision {
    #[default]
    Date,
    DateTime,
}

impl PartialEq for TimestampPrecision {
    fn eq(&self, other: &Self) -> bool {
        !matches!(
            (self, other),
            (TimestampPrecision::Date, TimestampPrecision::DateTime)
                | (TimestampPrecision::DateTime, TimestampPrecision::Date)
        )
    }
}
