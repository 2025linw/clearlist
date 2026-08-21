use std::borrow::Cow;

use serde::{Deserialize, Deserializer, de::Error as DeError};

use super::QueryDate;

pub fn deserialize_bool<'de, D>(deserialize: D) -> Result<bool, D::Error>
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

pub fn deserialize_iso8601daterange<'de, D, T>(deserialize: D) -> Result<[T; 2], D::Error>
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
