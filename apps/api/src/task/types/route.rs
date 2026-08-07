use serde::Deserialize;

use crate::{
    tag::types::TagID,
    types::{date::Start, date_query::QueryDateFilter, order::SortOrder},
};

use super::SortBy;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct URLQueryOpts {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,

    pub start: Option<QueryDateFilter<chrono::DateTime<chrono::Utc>>>,
    pub deadline: Option<QueryDateFilter<chrono::NaiveDate>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub tags: Option<Vec<TagID>>,
}

#[derive(Debug)]
pub struct CreateRequest {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<Start>,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<TagID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct UpdateRequest {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<Start>>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub position_key: Option<String>,
}
