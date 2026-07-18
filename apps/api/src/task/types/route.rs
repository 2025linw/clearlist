use serde::Deserialize;

use crate::types::{date_query::DateQueryFilter, order::SortOrder};

use super::{SortBy, TagID};

#[derive(Debug, Deserialize)]
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
