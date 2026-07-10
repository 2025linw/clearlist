use crate::{tag::types::SortBy, types::order::SortOrder};

#[derive(Debug, Default)]
pub struct Filter {
    pub category: Option<String>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);

        self
    }
}

#[derive(Debug, Default)]
pub struct Sort {
    pub sort: Option<(SortBy, SortOrder)>,
}

impl Sort {
    pub fn new(by: Option<SortBy>, order: SortOrder) -> Self {
        if let Some(by) = by {
            Self {
                sort: Some((by, order)),
            }
        } else {
            Self {
                sort: Some((SortBy::ID, order)),
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Pagination {
    pub fn new(limit: Option<usize>, offset: Option<usize>) -> Self {
        Self { limit, offset }
    }
}
