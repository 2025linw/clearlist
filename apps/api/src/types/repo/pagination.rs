#[derive(Debug, Default, Clone)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Pagination {
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
