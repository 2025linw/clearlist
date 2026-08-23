use std::str::FromStr;

use chrono::{DateTime, SecondsFormat, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::error::service::{Error, ValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Start {
    Date(chrono::NaiveDate),
    DateTime(chrono::DateTime<chrono::Utc>),
}

impl Start {
    pub fn has_time(&self) -> bool {
        matches!(self, Start::DateTime(_))
    }

    pub fn into_datetime_utc_with_tz(&self, tz: Tz) -> DateTime<Utc> {
        match self {
            Start::Date(naive_date) => naive_date
                .and_hms_opt(0, 0, 0)
                .expect("midnight should be valid")
                .and_local_timezone(tz)
                .single()
                .expect("midnight should be unambiguous")
                .to_utc(),
            Start::DateTime(date_time) => *date_time,
        }
    }
}

impl Default for Start {
    fn default() -> Self {
        Self::DateTime(chrono::Utc::now())
    }
}

impl std::fmt::Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Start::Date(naive_date) => write!(f, "{naive_date}"),
            Start::DateTime(date_time) => write!(
                f,
                "{}",
                date_time.to_rfc3339_opts(SecondsFormat::Secs, true)
            ),
        }
    }
}

impl FromStr for Start {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if let Ok(date) = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            return Ok(Self::Date(date));
        }
        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&s) {
            return Ok(Self::DateTime(datetime.to_utc()));
        }

        Err(Error::Validation(ValidationError::InvalidValue {
            field: "start",
            reason: "invalid start date format",
        }))
    }
}
