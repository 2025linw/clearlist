use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

// TODO: remove URLPagination
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct URLPagination {
    #[serde_as(as = "DisplayFromStr")]
    pub page: usize,
    #[serde_as(as = "DisplayFromStr")]
    pub limit: usize,
}

impl URLPagination {
    pub fn offset(&self) -> usize {
        (self.page - 1) * self.limit
    }
}

impl Default for URLPagination {
    fn default() -> Self {
        Self { page: 1, limit: 20 }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub struct SQLPagination {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl SQLPagination {
    pub fn new() -> Self {
        Self {
            limit: None,
            offset: None,
        }
    }

    pub fn with_limit(limit: u32) -> Self {
        Self {
            limit: Some(limit),
            offset: None,
        }
    }

    pub fn with_offset(offset: u32) -> Self {
        Self {
            limit: None,
            offset: Some(offset),
        }
    }

    pub fn limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    pub fn offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }
}
