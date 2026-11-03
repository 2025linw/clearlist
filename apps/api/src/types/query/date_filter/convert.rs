use chrono_tz::Tz;

use crate::{
    error::service::{RANGE_OVERSPECIFIED, ValidationError},
    types::{
        query::QueryDate,
        repo::{DateBound, DateFilter as RepoDateFilter},
    },
};

use super::{BracketInterval, DateFilter};

impl<T> DateFilter<T>
where
    T: QueryDate,
{
    pub fn try_into_with_tz(self, tz: Tz) -> Result<RepoDateFilter<T::Inner>, ValidationError> {
        match self {
            Self::Has(bool) => Ok(RepoDateFilter::Exists(bool)),
            Self::Exact(date) => Ok(RepoDateFilter::On(date.to_inner_with_tz(tz))),
            Self::BracketInterval(bracket_interval) => bracket_interval.try_into_with_tz(tz),
            Self::ISO8601Interval([start, end]) => Ok(RepoDateFilter::Range(
                DateBound::Inclusive(start.to_inner_with_tz(tz)),
                DateBound::Exclusive(end.to_inner_with_tz(tz)),
            )),
        }
    }
}

impl<T> BracketInterval<T>
where
    T: QueryDate,
{
    pub fn try_into_with_tz(self, tz: Tz) -> Result<RepoDateFilter<T::Inner>, ValidationError> {
        match self {
            // ne only
            BracketInterval {
                ne: Some(date),
                lt: None,
                lte: None,
                gt: None,
                gte: None,
            } => Ok(RepoDateFilter::NotOn(date.to_inner_with_tz(tz))),

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
                    .or_else(|| gte.map(DateBound::Inclusive))
                    .map(|bound| bound.map(|date| date.to_inner_with_tz(tz)));

                let end = lt
                    .map(DateBound::Exclusive)
                    .or_else(|| lte.map(DateBound::Inclusive))
                    .map(|bound| bound.map(|date| date.to_inner_with_tz(tz)));

                match (start, end) {
                    (Some(start), Some(end)) => Ok(RepoDateFilter::Range(start, end)),
                    (Some(start), None) => Ok(RepoDateFilter::StartRange(start)),
                    (None, Some(end)) => Ok(RepoDateFilter::EndRange(end)),
                    (None, None) => unreachable!(),
                }
            }
        }
    }
}
