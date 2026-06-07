use chrono::Utc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Start Date Type
///
/// This allows the conversion between date and datetimes into task start date/datetimes
///
/// This will serialize and deserialize to and from a 'YYYY-MM-DD` date string or ISO8601 datetime string
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[serde(tag = "type", content = "value")]
#[ts(export)]
pub enum Start {
    On(chrono::NaiveDate),
    At(chrono::DateTime<Utc>),
}

impl Start {
    /// Get Start as an On date, if it is not an On date, returns None
    pub fn as_on(&self) -> Option<chrono::NaiveDate> {
        match self {
            Self::On(date) => Some(*date),
            Self::At(_) => None,
        }
    }

    /// Get Start as an At date, if it is not an At date returns None
    pub fn as_at(&self) -> Option<chrono::DateTime<Utc>> {
        match self {
            Self::On(_) => None,
            Self::At(datetime) => Some(*datetime),
        }
    }
}
