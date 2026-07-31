use std::borrow::Cow;

use serde::{Deserialize, Deserializer, de::Error};

/// Trait to mark a type as a queryable date with DateQueryFilter
pub trait QueryDate: std::str::FromStr<Err: std::fmt::Display> + Sized {}

impl QueryDate for chrono::NaiveDate {}
impl QueryDate for chrono::DateTime<chrono::Utc> {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum QueryDateFilter<T: QueryDate> {
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

#[derive(Debug, Deserialize)]
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
