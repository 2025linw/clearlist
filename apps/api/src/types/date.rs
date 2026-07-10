//! # Date Range Filter
//!
//! This module contains types used for database date filtering

use serde::Deserialize;
use sqlx::QueryBuilder;
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, TS)]
#[ts(export, rename = "StartPrecision")]
pub enum StartPrecision {
    #[default]
    Date,
    DateTime,
}

#[derive(Debug)]
pub enum DateBound<T> {
    Exclusive(T),
    Inclusive(T),
}

#[derive(Debug)]
pub enum DateFilter<T> {
    Exists(bool),
    On(T),
    NotOn(T),
    StartRange(DateBound<T>),
    EndRange(DateBound<T>),
    Range(DateBound<T>, DateBound<T>),
}

impl<'a, T: 'a> DateFilter<T>
where
    T: Copy + sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres>,
{
    pub fn add_to_builder(&self, builder: &mut QueryBuilder<'a, sqlx::Postgres>) {
        match self {
            DateFilter::Exists(exists) => {
                if *exists {
                    builder.push("start_dt IS NOT NULL");
                } else {
                    builder.push("start_dt IS NULL");
                }
            }
            DateFilter::On(dt) => {
                builder.push("start_dt = ");
                builder.push_bind(*dt);
            }
            DateFilter::NotOn(dt) => {
                builder.push("start_dt <> ");
                builder.push_bind(*dt);
            }
            DateFilter::StartRange(date_bound) => match date_bound {
                DateBound::Exclusive(dt) => {
                    builder.push("start_dt > ");
                    builder.push_bind(*dt);
                }
                DateBound::Inclusive(dt) => {
                    builder.push("start_dt >= ");
                    builder.push_bind(*dt);
                }
            },
            DateFilter::EndRange(date_bound) => match date_bound {
                DateBound::Exclusive(dt) => {
                    builder.push("start_dt < ");
                    builder.push_bind(*dt);
                }
                DateBound::Inclusive(dt) => {
                    builder.push("start_dt <= ");
                    builder.push_bind(*dt);
                }
            },
            DateFilter::Range(start_bound, end_bound) => {
                match start_bound {
                    DateBound::Exclusive(dt) => {
                        builder.push("start_dt > ");
                        builder.push_bind(*dt);
                    }
                    DateBound::Inclusive(dt) => {
                        builder.push("start_dt >= ");
                        builder.push_bind(*dt);
                    }
                }
                match end_bound {
                    DateBound::Exclusive(dt) => {
                        builder.push("start_dt < ");
                        builder.push_bind(*dt);
                    }
                    DateBound::Inclusive(dt) => {
                        builder.push("start_dt <= ");
                        builder.push_bind(*dt);
                    }
                }
            }
        }
    }
}
