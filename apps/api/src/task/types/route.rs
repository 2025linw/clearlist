use serde::Deserialize;

use crate::{
    tag::types::TagID,
    types::{date::StartPrecision, date_query::DateQueryFilter, order::SortOrder},
};

use super::SortBy;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct URLQueryOpts {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,

    pub start: Option<DateQueryFilter<chrono::DateTime<chrono::Utc>>>,
    pub deadline: Option<DateQueryFilter<chrono::NaiveDate>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub tags: Option<Vec<TagID>>,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub start_precision: StartPrecision,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<TagID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateRequest {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub start_precision: Option<StartPrecision>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub position_key: Option<String>,
}
