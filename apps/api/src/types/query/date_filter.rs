pub mod convert;

mod deserialize_helpers;

use chrono_tz::Tz;
use serde::Deserialize;

use crate::types::start::Start;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DateFilter<T>
where
    T: QueryDate,
{
    /// Existence Filter
    #[serde(deserialize_with = "deserialize_helpers::deserialize_bool")]
    Has(bool),

    /// Exact Filter, has to match exactly
    Exact(T),

    /// Filter created by complex queries
    BracketInterval(BracketInterval<T>),

    /// Filter created by ISO8601 Interval format (<start>/<end>) === [start, end)
    #[serde(deserialize_with = "deserialize_helpers::deserialize_iso8601daterange")]
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

pub type ISO8601Interval<T> = [T; 2];

/// Trait to mark a type as a queryable date with DateQueryFilter
pub trait QueryDate: std::str::FromStr<Err: std::fmt::Display> + Sized {
    type Inner;

    const FIELD: &str;

    fn to_inner(&self) -> Self::Inner;

    fn to_inner_with_tz(&self, _: Tz) -> Self::Inner {
        self.to_inner()
    }
}

impl QueryDate for chrono::NaiveDate {
    type Inner = chrono::NaiveDate;

    const FIELD: &str = "deadline";

    fn to_inner(&self) -> Self::Inner {
        *self
    }
}

impl QueryDate for Start {
    type Inner = chrono::DateTime<chrono::Utc>;

    const FIELD: &str = "start";

    fn to_inner(&self) -> Self::Inner {
        self.into_datetime_utc_with_tz(Tz::UTC)
    }

    fn to_inner_with_tz(&self, tz: Tz) -> Self::Inner {
        self.into_datetime_utc_with_tz(tz)
    }
}
