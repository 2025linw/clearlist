use serde::Deserialize;

use crate::{
    tag::types::TagID,
    types::{field::Start, order::SortOrder, query::DateFilter},
};

use super::SortBy;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct URLQueryOpts {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    #[serde(alias = "sort")]
    pub sort_by: Option<SortBy>,
    #[serde(alias = "order")]
    pub sort_order: Option<SortOrder>,

    pub start: Option<DateFilter<Start>>,
    pub deadline: Option<DateFilter<chrono::NaiveDate>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub tags: Option<Vec<TagID>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<Start>,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Option<Vec<TagID>>,

    pub position_key: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<Start>>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub position_key: Option<String>,
}

impl UpdateRequest {
    pub fn is_noop(&self) -> bool {
        let Self {
            title,
            notes,
            start,
            deadline,
            tags,
            position_key,
        } = self;

        title.is_none()
            && notes.is_none()
            && start.is_none()
            && deadline.is_none()
            && tags.is_none()
            && position_key.is_none()
    }
}
