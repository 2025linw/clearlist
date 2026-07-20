use sqlx::QueryBuilder;

use crate::tag::types::TagCategoryID;

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub struct QueryOpts {
    pub filter: Filter,
    pub pagination: Pagination,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(category_id) = self.filter.category {
            builder.push(" AND category_id = ");
            builder.push_bind(category_id);
        }

        // Sort (forced)
        builder.push(" ORDER BY tc.position_key NULLS FIRST, t.position_key");

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
    pub category_id: Option<TagCategoryID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub label: Option<String>,
    pub category_id: Option<Option<TagCategoryID>>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(label) = self.label {
            separated.push("label = ");
            separated.push_bind_unseparated(label);
        }
        if let Some(category_id) = self.category_id {
            separated.push("category_id = ");
            separated.push_bind_unseparated(category_id);
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub struct Filter {
    pub category: Option<TagCategoryID>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: TagCategoryID) -> Self {
        self.category = Some(category);

        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone))]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Pagination {
    pub fn new(limit: Option<usize>, offset: Option<usize>) -> Self {
        Self { limit, offset }
    }
}
