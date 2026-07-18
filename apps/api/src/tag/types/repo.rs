use sqlx::QueryBuilder;

use crate::types::order::SortOrder;

use super::SortBy;

#[derive(Debug, Default)]
pub struct QueryOpts {
    pub filter: Filter,
    pub sort: Sort,
    pub pagination: Pagination,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(filter) = self.filter.category {
            builder.push(" AND category = ");
            builder.push_bind(filter);
        }

        // Sort
        if let Some((by, order)) = self.sort.sort {
            builder.push(format!(" ORDER BY {} {}", by, order));
        } else {
            builder.push(" ORDER BY id ASC");
        }

        // Pagination
        if let Some(limit) = self.pagination.limit {
            builder.push(format!(" LIMIT {limit}"));
        }
        if let Some(offset) = self.pagination.offset {
            builder.push(format!(" OFFSET {offset}"));
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct CreateModel {
    pub label: String,
    pub category: Option<String>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub label: Option<String>,
    pub category: Option<Option<String>>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(label) = self.label {
            separated.push("label = ");
            separated.push_bind_unseparated(label);
        }
        if let Some(category) = self.category {
            separated.push("category = ");
            separated.push_bind_unseparated(category);
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}

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
