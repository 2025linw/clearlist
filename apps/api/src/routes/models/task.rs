//! # Route Task Model
//!
//! This module contains types used for querying and filtering

use serde::Deserialize;

use super::{DateFilter, Pagination, SortOrder};

/// Task Filter Model
///
/// This represents the url parameter fields for filtering Tasks queried
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    #[serde(flatten)]
    pub pagination: Pagination,

    #[serde(default, rename = "sort")]
    pub sort_by: SortBy,
    #[serde(default, rename = "order")]
    pub sort_order: SortOrder,

    #[serde(rename = "start")]
    pub start_date: Option<DateFilter>,
    pub deadline: Option<DateFilter>,

    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub deleted: bool,
}

/// Task Sort Type
///
/// This represents all the values that it is possible to sort Tasks by
#[derive(Debug, Default, Deserialize)]
pub enum SortBy {
    Created,
    #[default]
    Updated,
    Title,
    Start,
    #[serde(alias = "due")]
    Deadline,
}
