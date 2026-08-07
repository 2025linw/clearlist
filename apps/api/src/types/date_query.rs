use std::borrow::Cow;

use serde::{Deserialize, Deserializer, de::Error as DeError};

use crate::error::service::{RANGE_OVERSPECIFIED, ValidationError};

use super::date::{DateBound, DateFilter};

/// Trait to mark a type as a queryable date with DateQueryFilter
pub trait QueryDate: std::str::FromStr<Err: std::fmt::Display> + Sized {
    const FIELD: &str;
}

impl QueryDate for chrono::DateTime<chrono::Utc> {
    const FIELD: &str = "start";
}
impl QueryDate for chrono::NaiveDate {
    const FIELD: &str = "deadline";
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum QueryDateFilter<T>
where
    T: QueryDate,
{
    /// Existence Filter
    #[serde(deserialize_with = "deserialize_bool")]
    Has(bool),

    /// Exact Filter, has to match exactly
    Exact(T),

    /// Filter created by complex queries
    BracketInterval(BracketInterval<T>),

    /// Filter created by ISO8601 Interval format (<start>/<end>)
    #[serde(deserialize_with = "deserialize_iso8601daterange")]
    ISO8601Interval(ISO8601Interval<T>),
}

#[derive(Debug, Default, Deserialize)]
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

fn deserialize_bool<'de, D>(deserialize: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Cow<'_, str> = Deserialize::deserialize(deserialize)?;

    if s.trim().is_empty() {
        return Err(D::Error::custom("Value cannot be empty"));
    }

    let bool = s
        .parse::<bool>()
        .map_err(|_| D::Error::custom("Invalid bool value"))?;

    Ok(bool)
}

fn deserialize_iso8601daterange<'de, D, T>(deserialize: D) -> Result<[T; 2], D::Error>
where
    D: Deserializer<'de>,
    T: QueryDate,
{
    let s: Cow<'_, str> = Deserialize::deserialize(deserialize)?;

    if s.trim().is_empty() {
        return Err(D::Error::custom("Date range cannot be empty"));
    }

    let mut parts = s.split('/');

    let start = parts
        .next()
        .ok_or_else(|| D::Error::custom("Missing start date"))?
        .parse::<T>()
        .map_err(D::Error::custom)?;

    let end = parts
        .next()
        .ok_or_else(|| D::Error::custom("Missing end date"))?
        .parse::<T>()
        .map_err(D::Error::custom)?;

    if parts.next().is_some() {
        return Err(D::Error::custom("Found too many dates in range"));
    }

    Ok([start, end])
}

impl<T> TryFrom<QueryDateFilter<T>> for DateFilter<T>
where
    T: QueryDate,
{
    type Error = ValidationError;

    fn try_from(value: QueryDateFilter<T>) -> Result<Self, Self::Error> {
        match value {
            QueryDateFilter::Has(bool) => Ok(Self::Exists(bool)),
            QueryDateFilter::Exact(date) => Ok(Self::On(date)),
            QueryDateFilter::BracketInterval(bracket_interval) => {
                match bracket_interval {
                    // ne only
                    BracketInterval {
                        ne: Some(date),
                        lt: None,
                        lte: None,
                        gt: None,
                        gte: None,
                    } => Ok(DateFilter::NotOn(date)),

                    // invalid: ne with anything else
                    BracketInterval {
                        ne: Some(_),
                        lt,
                        lte,
                        gt,
                        gte,
                    } if lt.is_some() || lte.is_some() || gt.is_some() || gte.is_some() => {
                        Err(ValidationError::InvalidValue {
                            field: T::FIELD,
                            reason: RANGE_OVERSPECIFIED,
                        })
                    }

                    // invalid lower bound
                    BracketInterval {
                        gt: Some(_),
                        gte: Some(_),
                        ..
                    } => Err(ValidationError::InvalidValue {
                        field: T::FIELD,
                        reason: RANGE_OVERSPECIFIED,
                    }),

                    // invalid upper bound
                    BracketInterval {
                        lt: Some(_),
                        lte: Some(_),
                        ..
                    } => Err(ValidationError::InvalidValue {
                        field: T::FIELD,
                        reason: RANGE_OVERSPECIFIED,
                    }),

                    // valid range
                    BracketInterval {
                        gt, gte, lt, lte, ..
                    } => {
                        let start = gt
                            .map(DateBound::Exclusive)
                            .or_else(|| gte.map(DateBound::Inclusive));

                        let end = lt
                            .map(DateBound::Exclusive)
                            .or_else(|| lte.map(DateBound::Inclusive));

                        match (start, end) {
                            (Some(start), Some(end)) => Ok(DateFilter::Range(start, end)),
                            (Some(start), None) => Ok(DateFilter::StartRange(start)),
                            (None, Some(end)) => Ok(DateFilter::EndRange(end)),
                            (None, None) => unreachable!(),
                        }
                    }
                }
            }
            QueryDateFilter::ISO8601Interval([start, end]) => Ok(DateFilter::Range(
                DateBound::Inclusive(start),
                DateBound::Exclusive(end),
            )),
        }
    }
}
