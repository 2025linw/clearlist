use serde::Deserialize;

use crate::types::order::SortOrder;

use super::SortBy;

#[derive(Debug, Deserialize)]
pub struct URLQueryOpts {
    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,

    pub category: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Default))]
pub struct CreateRequest {
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateRequest {
    pub label: Option<String>,
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}
