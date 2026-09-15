use crate::{
    error::service::{RANGE_OVERSPECIFIED, ValidationError},
    types::{
        field::{DateBound, DateFilter as DateFilterField},
        query::{BracketInterval, DateFilter as DateFilterQuery, QueryDate},
    },
};

impl<T> TryFrom<DateFilterQuery<T>> for DateFilterField<T>
where
    T: QueryDate,
{
    type Error = ValidationError;

    fn try_from(value: DateFilterQuery<T>) -> Result<Self, Self::Error> {
        match value {
            DateFilterQuery::Has(bool) => Ok(Self::Exists(bool)),
            DateFilterQuery::Exact(date) => Ok(Self::On(date)),
            DateFilterQuery::BracketInterval(bracket_interval) => bracket_interval.try_into(),
            DateFilterQuery::ISO8601Interval([start, end]) => Ok(DateFilterField::Range(
                DateBound::Inclusive(start),
                DateBound::Exclusive(end),
            )),
        }
    }
}

impl<T> TryFrom<BracketInterval<T>> for DateFilterField<T>
where
    T: QueryDate,
{
    type Error = ValidationError;

    fn try_from(value: BracketInterval<T>) -> Result<Self, Self::Error> {
        match value {
            // ne only
            BracketInterval {
                ne: Some(date),
                lt: None,
                lte: None,
                gt: None,
                gte: None,
            } => Ok(DateFilterField::NotOn(date)),

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
                    (Some(start), Some(end)) => Ok(DateFilterField::Range(start, end)),
                    (Some(start), None) => Ok(DateFilterField::StartRange(start)),
                    (None, Some(end)) => Ok(DateFilterField::EndRange(end)),
                    (None, None) => unreachable!(),
                }
            }
        }
    }
}
