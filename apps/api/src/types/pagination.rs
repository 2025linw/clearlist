use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Pagination {
    #[serde_as(as = "DisplayFromStr")]
    pub page: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub limit: i64,
}

impl Pagination {
    pub fn offset(self) -> i64 {
        (self.page - 1) * self.limit
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, limit: 20 }
    }
}
