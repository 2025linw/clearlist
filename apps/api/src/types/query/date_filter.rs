mod helper;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Deserialize;

use crate::types::field::Start;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DateFilter<T>
where
    T: QueryDate,
{
    /// Existence Filter
    #[serde(deserialize_with = "helper::deserialize_bool")]
    Has(bool),

    /// Exact Filter, has to match exactly
    Exact(T),

    /// Filter created by complex queries
    BracketInterval(BracketInterval<T>),

    /// Filter created by ISO8601 Interval format (<start>/<end>) === [start, end)
    #[serde(deserialize_with = "helper::deserialize_iso8601daterange")]
    ISO8601Interval(ISO8601Interval<T>),
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct BracketInterval<T> {
    #[serde(alias = "<>")]
    pub ne: Option<T>,
    #[serde(alias = "<")]
    pub lt: Option<T>,
    pub lte: Option<T>,
    #[serde(alias = ">")]
    pub gt: Option<T>,
    pub gte: Option<T>,
}

impl BracketInterval<Start> {
    pub fn into_datetime_utc_with_tz(self, tz: Tz) -> BracketInterval<DateTime<Utc>> {
        BracketInterval {
            ne: self.ne.map(|start| start.into_datetime_utc_with_tz(tz)),
            lt: self.lt.map(|start| start.into_datetime_utc_with_tz(tz)),
            lte: self.lte.map(|start| start.into_datetime_utc_with_tz(tz)),
            gt: self.gt.map(|start| start.into_datetime_utc_with_tz(tz)),
            gte: self.gte.map(|start| start.into_datetime_utc_with_tz(tz)),
        }
    }
}

pub type ISO8601Interval<T> = [T; 2];

/// Trait to mark a type as a queryable date with DateQueryFilter
pub trait QueryDate: std::str::FromStr<Err: std::fmt::Display> + Sized {
    const FIELD: &str;
}

impl QueryDate for chrono::NaiveDate {
    const FIELD: &str = "deadline";
}
impl QueryDate for chrono::DateTime<chrono::Utc> {
    const FIELD: &str = "start";
}
impl QueryDate for Start {
    const FIELD: &str = "start";
}
